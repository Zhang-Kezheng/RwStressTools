import Database from "@tauri-apps/plugin-sql";

let dataBase :Database=  null

Database.load('sqlite:RwStressTools.db').then(async (db) => {
    dataBase = db;
    await initTable()
})
async function  initTable() {
    await dataBase.execute(`
        CREATE TABLE IF NOT EXISTS TagModel
        (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            mac             varchar(50) NOT NULL,
            voltage         double,
            tamper          boolean,
            button          boolean,
            shock           boolean,
            heartRate       integer,
            bloodPressureH  integer,
            bloodPressureL  integer,
            bloodOxygen     integer,
            bodyTemperature integer,
            stepCount       integer,
            sleepState      integer,
            deepSleepTime   integer,
            lightSleepTime  integer,
            rssi            integer,
            recordId        integer,
            deviceId        integer,
            date            date

        )
    `)
}
export {dataBase}



export class TagModel{
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
    recordId:number;
    deviceId:string;
    date: Date= Date()
}