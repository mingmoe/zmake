module;
#include <string>
#include <v8-exception.h>
#include <memory>
#include <vector>
#include <v8-local-handle.h>
#include <v8.h>
#include <v8-platform.h>
#include <v8-initialization.h>
#include <v8-isolate.h>
#include <v8-primitive.h>
#include <v8-context.h>
#include <libplatform/libplatform.h>
#include <mutex>
#include <v8-array-buffer.h>
#include <v8-object.h>
#include <string_view>
#include <optional>
#include <fmt/core.h>
#include <tl/expected.hpp>
export module Moe.ZMake:V8;
import :Engine;

namespace MoeOrg::ZMake{

    using ExpectedStr = tl::expected<std::string, std::string>;

    export class V8Engine : public EngineInterface{
        public:

            V8Engine(){
                initilize();

                v8::Isolate::CreateParams params{};

                params.array_buffer_allocator =
                    v8::ArrayBuffer::Allocator::NewDefaultAllocator();

                isolate = v8::Isolate::New(params);
            }

            virtual ~V8Engine(){
            }
            
            virtual std::string getEngineDescription() override{
                return fmt::format("v8 ver {}",v8::V8::GetVersion());
            }
            
            virtual tl::expected<std::string,std::string> eval(std::string_view source,
                    std::string_view fileName) override {
                v8::Local<v8::String> src = toV8String(source);
                v8::Local<v8::String> name = this->toV8String(fileName);

                return eval(src,name);
            }


        private:
            v8::Isolate* isolate;

            static void initilize(){
                static std::mutex lock{};
                static bool initilized = false;
                std::lock_guard<std::mutex> guard{lock};

                if(initilized){
                    return;
                }

                initilized = true;

                std::unique_ptr<v8::Platform> platform = v8::platform::NewDefaultPlatform();
                v8::V8::InitializePlatform(platform.get());
                v8::V8::Initialize();
            }

            static std::string toString(const v8::String::Utf8Value& value) {
                  return *value ? *value : "<string conversion failed>";
            }

            v8::Local<v8::String> toV8String(std::string_view string){
               v8::Local<v8::String> source;

                if(v8::String::NewFromUtf8(this->isolate, string.data()).ToLocal(&source)){
                    return v8::String::NewFromUtf8Literal(this->isolate, "<string conversion failed>");
                }

                return source;
            }

            std::string reportException(v8::TryCatch& tryCatch){
                v8::HandleScope handle_scope(isolate);
                v8::String::Utf8Value exception(isolate, tryCatch.Exception());
                
                v8::Local<v8::Message> message = tryCatch.Message();

                if(message.IsEmpty()){
                    return toString(exception);
                }
                // print (file name):(line name):(message)
                std::vector<char> errorMessage ( 512 );

                v8::String::Utf8Value filename(isolate,
                                       message->GetScriptOrigin().ResourceName());
                v8::Local<v8::Context> context(isolate->GetCurrentContext());
                auto fileName = toString(filename);
                int linenum = message->GetLineNumber(context).FromJust();

                fmt::format_to(std::back_inserter(errorMessage),"at {}(line {}): {}\n",fileName,linenum,toString(exception));

                // Print line of source code.
                v8::String::Utf8Value sourceline(
                    isolate, message->GetSourceLine(context).ToLocalChecked());
                std::string sourcelineString = toString(sourceline);

                fmt::format_to(std::back_inserter(errorMessage), "{}\n",sourcelineString);
                // Print wavy underline (GetUnderline is deprecated).
                int start = message->GetStartColumn(context).FromJust();
                for (int i = 0; i < start; i++) {
                  errorMessage.push_back(' ');
                }
                int end = message->GetEndColumn(context).FromJust();
                for (int i = start; i < end; i++) {
                  errorMessage.push_back('^');
                }
                errorMessage.push_back('\n');
                // print stack trace
                v8::Local<v8::Value> stack_trace_string;
                if (tryCatch.StackTrace(context).ToLocal(&stack_trace_string) &&
                    stack_trace_string->IsString() &&
                stack_trace_string.As<v8::String>()->Length() > 0) {
                  v8::String::Utf8Value stack_trace(isolate, stack_trace_string);
                  std::string err = toString(stack_trace);
                  fmt::format_to(std::back_inserter(errorMessage),"{}",err);
                }

                return std::string(errorMessage.data(),errorMessage.size());
            }

            tl::expected<std::string,std::string> eval(
                    v8::Local<v8::String> source,
                    v8::Local<v8::Value> scriptLocal){
                v8::HandleScope handleScope { this->isolate };
                v8::TryCatch tryCatch{ isolate };
                v8::ScriptOrigin origin{ scriptLocal };
                v8::Local<v8::Context> context(isolate->GetCurrentContext());
                v8::Local<v8::Script> script;

                if(v8::Script::Compile(context , source, &origin).ToLocal(&script)){
                    return ExpectedStr(tl::unexpected(reportException(tryCatch)));
                }

                v8::Local<v8::Value> result;
                if(!script->Run(context).ToLocal(&result)){
                    return ExpectedStr(tl::unexpected(reportException(tryCatch)));
                }

                // FIXME: process result
                
                v8::String::Utf8Value str(this->isolate, result);

                return ExpectedStr(toString(str));
            }

    };
}
