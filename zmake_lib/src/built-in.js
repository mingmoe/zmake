
import * as zmake from "zmake";

if(zmake.debug){
    console.trace("zmake.debug == true")
    console.trace("zmake.workingDirectory:%s",zmake.workingDirectory)
    console.trace("zmake.cacheDirectory:%s",zmake.cacheDirectory)
    console.trace("zmake.zmakeDirectory:%s",zmake.zmakeDirectory)
}

// import user's file
await import('./zmake.ts'); // suffixes does not matters,we will try .ts and other common options.
