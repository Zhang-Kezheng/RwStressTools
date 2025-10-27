import * as ByteBuffer from "bytebuffer";
import {listen} from "@kuyoonjo/tauri-plugin-udp";

export class AoAGateway {
     header=0x02030405;
    length=null;
    dev_id=new Uint8Array([0x02, 0x03, 0x04, 0x05, 0x06, 0x07]);
    cmd=0x01;
    sn=null;
    jiami=1;
    data:Uint8Array<ArrayBuffer>=null;
    check_sum=null;
    count=0
    static getInstance(data:ArrayBuffer):AoAGateway{
        let aoa = new AoAGateway();
        let buf = ByteBuffer.wrap(data)
        let header=buf.readInt()
        if (header !==0x02030405){
            return null;
        }
        aoa.length=buf.readShort()
        aoa.dev_id=new Uint8Array(buf.readBytes(6).toArrayBuffer())
        aoa.cmd=buf.readInt8()
        aoa.sn=buf.readInt8()
        aoa.jiami=buf.readInt8()
        aoa.data=new Uint8Array<ArrayBuffer>(buf.readBytes(aoa.length-16).toArrayBuffer())
        aoa.check_sum=buf.readInt8()
        return  aoa;
    }
}