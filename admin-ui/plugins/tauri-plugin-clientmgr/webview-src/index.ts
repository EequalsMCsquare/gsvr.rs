import { EventCallback, listen, UnlistenFn } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri'

interface tag_fmsg {
  id: number,
  payload: string
}

interface tag_nmsg {
  username: string,
  payload: string,
}

function add_fclient(id: Number): Promise<null> {
  return invoke('plugin:clientmgr|add_fclient', { playerId: id });
}

function drop_fclient(id: Number): Promise<null> {
  return invoke('plugin:clientmgr|drop_fclient', { playerId: id });
}

function fclient_request(id: Number, content: string): Promise<string> {
  return invoke('plugin:clientmgr|fclient_request', { playerId: id, content });
}

function fclient_listen_reply(cb: EventCallback<tag_fmsg>): Promise<UnlistenFn> {
  return listen('recv_fscmsg', cb);
}

export default {
  add_fclient,
  drop_fclient,
  fclient_request,
  fclient_listen_reply,
}