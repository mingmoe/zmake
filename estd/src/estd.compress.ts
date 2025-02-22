
// @ts-ignore
import * as compress from "estd.internal.compress";

export async function unzip(source:string,destination:string)
{
    compress.unzip(source,destination);
}

export async function untar(source:string,destination:string){
    compress.untar(source,destination);
}
