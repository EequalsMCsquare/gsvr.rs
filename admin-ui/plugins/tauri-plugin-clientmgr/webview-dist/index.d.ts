import { EventCallback, UnlistenFn } from '@tauri-apps/api/event';
interface tag_fmsg {
    id: number;
    payload: string;
}
declare function add_fclient(id: Number): Promise<null>;
declare function drop_fclient(id: Number): Promise<null>;
declare function fclient_request(id: Number, content: string): Promise<string>;
declare function fclient_listen_reply(cb: EventCallback<tag_fmsg>): Promise<UnlistenFn>;
declare const _default: {
    add_fclient: typeof add_fclient;
    drop_fclient: typeof drop_fclient;
    fclient_request: typeof fclient_request;
    fclient_listen_reply: typeof fclient_listen_reply;
};
export default _default;
