import * as ByteBuffer from "bytebuffer";
import {AoAGateway} from "./AoAGateway";
import {TagModel} from "../db";

export class AoATag{
    mac=new Uint8Array([0xa6,0x2b,0x3c,0x00,0x1a,0x63])
    length=0x1e
    fix=0xff
    manufacturerId=0x0d00
    packageId=0x04
    command=0x09
    userData=new Uint8Array([0x40,0x01,0x5D])
    crc=0x00
    dFField=new Uint8Array([0x2F,0x61,0xAC,0xCC,0x27,0x45,0x67,0xF7,0xDB
            ,0x34,0xC4,0x03,0x8E,0x5C,0x0B,0xAA,0x97,0x30,0x56,0xE6])
    rssi=0x04
    static getInstance(data:ArrayBuffer):AoATag{
        if (data.byteLength!=38){
            console.log("垃圾数据，丢弃")
            return null
        }
        let buf=ByteBuffer.wrap(data)
        let aoaTag=new AoATag();
        aoaTag.mac=new Uint8Array(buf.readBytes(6).toArrayBuffer())
        aoaTag.length=buf.readUint8()
        aoaTag.fix=buf.readUint8()
        aoaTag.manufacturerId=buf.readShort()
        aoaTag.packageId=buf.readUint8()
        aoaTag.command=buf.readUint8()
        aoaTag.userData=new Uint8Array(buf.readBytes(3).toArrayBuffer())
        aoaTag.crc=buf.readShort()
        aoaTag.dFField=new Uint8Array(buf.readBytes(20).toArrayBuffer())
        aoaTag.rssi=buf.readInt8()
        return aoaTag;
    }
}