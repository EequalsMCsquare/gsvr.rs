import { invoke } from '@tauri-apps/api/tauri'

function options(): Promise<string> {
  return invoke('plugin:pbhint|get_options')
}

function hint(pbName: string): Promise<string> {
  return invoke('plugin:pbhint|try_hint', { key: pbName })
}

export default {
  options,
  hint
}