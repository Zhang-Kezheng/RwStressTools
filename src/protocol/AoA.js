import ByteBuffer from "bytebuffer";
export class AoA {
     header=0x02030405;
    length=null;
    devId=new Uint8Array([0x02, 0x03, 0x04, 0x05, 0x06, 0x07]);
    cmd=0x01;
    sn=null;
    jiami=1;
    data=null;
    checkSum=null;

    static getInstance(data){
        let aoa = new AoA();
        let buf = new ByteBuffer(data.length);
        data.forEach(item=>{
            buf.writeInt8(item);
        })
        buf.flip()
        let header=buf.readInt()
        if (header !==0x02030405){
            return null;
        }
        aoa.length=buf.readShort()
        aoa.devId=buf.readBytes(6).toArrayBuffer()
        aoa.cmd=buf.readInt8()
        aoa.sn=buf.readInt8()
        aoa.jiami=buf.readInt8()
        aoa.data=buf.readBytes(aoa.length-16).toArrayBuffer()
        aoa.checkSum=buf.readInt8()
        return  aoa;
    }
}