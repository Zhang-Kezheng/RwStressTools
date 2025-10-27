import * as ByteBuffer from "bytebuffer";

import {AoAGateway} from "../protocol/AoAGateway.js";
import {AoATag} from "../protocol/AoATag.js";
import { AoaTagVo} from "../vo/vo.js";
self.onmessage = function(message_event) {
    const aoAGateway=AoAGateway.getInstance(message_event.data);
    let buf = ByteBuffer.wrap(aoAGateway.data)
    let count=buf.readUint8()
    if (aoAGateway.data.length!==count*38+1){
        console.log("垃圾数据，丢弃")
    }
    const aoaTagVos=[]
    for (let i = 0; i < count; i++) {
        let data =buf.readBytes(38).toArrayBuffer()
        let aoaTag=AoATag.getInstance(data)
        aoaTagVos.push(transform(aoaTag))
    }
    const mac =formatMac(aoAGateway.dev_id)
    postMessage({mac,aoaTagVos})
}
function transform(aoaTag){
    const aoaTagVo=new AoaTagVo()
    aoaTagVo.mac=formatMac(aoaTag.mac)
    aoaTagVo.rssi=aoaTag.rssi
    switch (aoaTag.command){
        case 0x09:
            aoaTagVo.voltage=Number((aoaTag.userData[2]*6.6/255).toFixed(2))
            aoaTagVo.tamper=(aoaTag.userData[0]>>5 & 0x01)===1
            aoaTagVo.button=(aoaTag.userData[0]>>4 & 0x01)===1
            aoaTagVo.shock=(aoaTag.userData[0]>>3 & 0x01)===1
            break
        case 0x0A:
            aoaTagVo.heartRate = aoaTag.userData[0]
            aoaTagVo.bloodPressureH=aoaTag.userData[1]
            aoaTagVo.bloodPressureL=aoaTag.userData[2]
            break
        case 0x0B:
            aoaTagVo.bloodOxygen = aoaTag.userData[0]
            break
        case 0x0C:
            const byteBuffer=ByteBuffer.wrap(aoaTag.userData)
            aoaTagVo.bodyTemperature = byteBuffer.readUint8()
            aoaTagVo.stepCount =byteBuffer.readUint16()
            break
        case 0x0D:
            aoaTagVo.sleepState = aoaTag.userData[0]
            aoaTagVo.lightSleepTime = aoaTag.userData[1]
            aoaTagVo.deepSleepTime = aoaTag.userData[2]
            break
    }
    return aoaTagVo
}
function formatMac(arr) {
    return Array.from(arr).map(num => {
        // 将数字转为16进制，toUpperCase() 可选（转为大写）
        let hex = num.toString(16).padStart(2, '0');
        // 处理超过2位的情况（如数字大于255时，取后两位）
        return hex.slice(-2).toUpperCase();
    }).join(':');
}