import { HintPair } from "pair"

type HintData = {
    pbnames: string[],
    pbids: number[],
    pairs: HintPair[]
}

class Hint {
    inner: HintData
    name2payload: Map<string, string>

    constructor(data: HintData) {
        this.inner = data;
        this.name2payload = new Map();
        data.pairs.forEach(v => {
            this.name2payload.set(v.name, v.payload);
        });
    }

    public get names(): string[] {
        return this.inner.pbnames.filter(v => (v.startsWith("cs") || v.startsWith("Cs")) &&
            ["CsLogin", "CsFastLogin"].findIndex(v2 => v2 === v) === -1);
    }

    public get ids(): number[] {
        return this.inner.pbids;
    }

    public get_payload(name: string): string | undefined {
        return this.name2payload.get(name)
    }
}

export { Hint };
export type {
    HintData
}