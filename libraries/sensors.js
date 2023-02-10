// const {execFile, execFileSync} = require("child_process");
// const path = require("path");
const ActiveX = require("winax");

class Sensors {
    constructor() {
        this.conn = new ActiveX.Object("WbemScripting.SWbemLocator");
        this.svr = this.conn.ConnectServer(".", "root\\LibreHardwareMonitor");
    }

    query(queryString) {
        const results = [];
        const queryResponse = this.svr.ExecQuery(queryString);
        for (let i = 0; i < queryResponse.Count; i += 1) {
          const properties = queryResponse.ItemIndex(i).Properties_;
          let count = properties.Count;
          const propEnum = properties._NewEnum;
          const obj = {};
          while (count) {
            count -= 1;
            const prop = propEnum.Next(1);
            obj[prop.Name] = prop.Value;
          }
          results.push(obj);
        }
    
        return results;
      }

    // updateSensors() {
    //     execFile(path.join(__dirname, "sensors", "capellix-lcd-cli-sensors", "bin", "Release", "net6.0", "capellix-lcd-cli-sensors.exe"), (error, stdout, stderr) => {
    //         this.sensordata = JSON.parse(stdout);
    //     });
    // }

    // updateSensorsSync() {
    //     this.sensordata = JSON.parse(execFileSync(path.join(__dirname, "sensors", "capellix-lcd-cli-sensors", "bin", "Release", "net6.0", "capellix-lcd-cli-sensors.exe")));
    // }

    // updateSensorsSyncAlt() {
    //     this.sensordata = JSON.parse(execFileSync(path.join(__dirname, "sensors", "capellix-lcd-cli-sensors", "bin", "Release", "net6.0", "capellix-lcd-cli-sensors.exe")));
    // }

    // startUpdating() {
    //     this.timer = setInterval(this.updateSensors, 500);
    // }

    // stopUpdating() {
    //     clearTimeout(this.updateSensors);
    // }

    fetchSensors() {
        return {};
    }
}

module.exports = Sensors;