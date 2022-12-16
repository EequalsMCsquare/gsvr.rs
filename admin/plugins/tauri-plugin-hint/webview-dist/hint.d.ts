import { HintPair } from "pair";
type HintData = {
    pbnames: string[];
    pbids: number[];
    pairs: HintPair[];
};
declare class Hint {
    inner: HintData;
    name2payload: Map<string, string>;
    constructor(data: HintData);
    get names(): string[];
    get ids(): number[];
    get_payload(name: string): string | undefined;
}
export { Hint };
export type { HintData };
