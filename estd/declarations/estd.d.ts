
declare module 'estd' {
    /**
     * check if the module is loaded
     * @param name the module name
     * @returns true if the module is loaded
     */
    export function hasModule(name: "fs" | "console"): boolean;
}
