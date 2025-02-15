
// @ts-ignore
import * as fs from "estd.internal.fs";

/**
 * read a file as string from filesystem
 * @param path the path of file
 */
export async function readFile(path:string):Promise<string>{
    return await fs.readFileAsync(path)
}

/**
 * write the file to filesystem.If the file exists, it will be overwritten.
 * If the file does not exist, it will be created.
 * @param path the path of the file
 * @param content the content to be written
 */
export async function writeFile(path:string,content:string):Promise<void>{
    return await fs.writeFileAsync(path,content);
}
