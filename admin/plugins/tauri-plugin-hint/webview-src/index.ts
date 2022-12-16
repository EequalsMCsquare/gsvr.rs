import { Hint, HintData } from "hint";
import { invoke } from "@tauri-apps/api";

let _hint: Hint;


function useHint(): Promise<Hint> {
  return new Promise<Hint>((resolve, reject) => {
    if (_hint === undefined) {
      invoke<string>("plugin:hint|get_hint").then(res => {
        const hintData: HintData = JSON.parse(res);
        _hint = new Hint(hintData);
        resolve(_hint);
      }).catch(err => {
        reject(err)
      });
    } else {
      resolve(_hint)
    }
  });
}

export {
  useHint
}