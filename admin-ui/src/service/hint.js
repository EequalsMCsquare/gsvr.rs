import { invoke } from "@tauri-apps/api/tauri";

function getOptions() {
    return invoke("plugin:pbhint|get_options");
}

function tryHint({key}) {
    return invoke("plugin:pbhint|try_hint", {key});
}

export default {
    getOptions, tryHint
}