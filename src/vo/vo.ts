

export interface PageResponse<T>{
    data: Array<T>;
    total: number;
}

export class AoaTagVo{
    id:number;
    mac:string;
    voltage:number;
    tamper:boolean;
    button:boolean;
    shock:boolean;
    heartRate:number;
    bloodPressureH:number;
    bloodPressureL:number;
    bloodOxygen:number;
    bodyTemperature:number;
    stepCount:number;
    sleepState:number;
    deepSleepTime:number;
    lightSleepTime:number;
    rssi:number;
}


export type AoaGatewayVo ={
    mac: string;
    tag_count: number;
    total: number;
}