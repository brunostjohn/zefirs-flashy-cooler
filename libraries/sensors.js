const {execSync} = require("child_process");
const ActiveX = require("winax");

class Sensors {
    constructor() {
        this.libreRunning = true;
        try {
            execSync("tasklist | findstr \"LibreHardwareMonitor.exe\"")
        } catch {
            this.libreRunning = false
        };
        if(this.libreRunning) {
          this.conn = new ActiveX.Object("WbemScripting.SWbemLocator");
          this.svr = this.conn.ConnectServer(".", "root\\LibreHardwareMonitor");
        }
    }

    query(queryString) {
        if (this.libreRunning) {
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
        } else {
          return {value: "0.0", unit: "unit"}
        }
    }

    fetchSensors(path) {
        return {};
    }
}

module.exports = Sensors;