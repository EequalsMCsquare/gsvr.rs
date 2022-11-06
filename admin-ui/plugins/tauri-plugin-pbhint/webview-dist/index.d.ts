declare function options(): Promise<string>;
declare function hint(pbName: string): Promise<string>;
declare const _default: {
    options: typeof options;
    hint: typeof hint;
};
export default _default;
