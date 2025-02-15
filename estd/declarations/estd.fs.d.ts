/**
 * read a file as string from filesystem
 * @param path the path of file
 */
export declare function readFile(path: string): Promise<string>;
/**
 * write the file to filesystem.If the file exists, it will be overwritten.
 * If the file does not exist, it will be created.
 * @param path the path of the file
 * @param content the content to be written
 */
export declare function writeFile(path: string, content: string): Promise<void>;
