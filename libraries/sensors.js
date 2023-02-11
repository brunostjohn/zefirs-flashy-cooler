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

    listHardware() {
        if(this.libreRunning){
            let results = this.query("Select Name, Identifier From Hardware");
            let ret = [];
            results.forEach((result) => {
                const finalObject = {
                    path: result.Identifier,
                    name: result.Name
                };
                ret.push(finalObject);
            });
            return ret;
        } else {
            return [{path: "/no-sensors/0", name: "Check if LibreHardwareMonitor is running."}];
        }
    }

    listSensorTypes(hardwarePath) {
        if(this.libreRunning){
            let results = this.query("Select SensorType From Sensor Where Parent = \"" + hardwarePath + "\"");
            let ret = [];
            results.forEach((result) => {
                if(!ret.includes(result.SensorType)) {
                    ret.push(result.SensorType);
                }
            });
            return ret;
        } else {
            return ["No Sensors"];
        }
    }

    listSensorsByType(hardwarePath, sensorType) {
        if(this.libreRunning) {
            let results = this.query("Select Identifier, Name From Sensor Where Parent = \"" + hardwarePath +"\" And SensorType = \"" + sensorType + "\"");
            let ret = [];
            results.forEach((result) => {
                const finalObject = {
                    path: result.Identifier,
                    name: result.Name
                };
                ret.push(finalObject);
            });
            return ret;
        } else {
            return [{path: "/no-sensors/0", name: "Check if LibreHardwareMonitor is running"}];
        }
    }

    getSensorValueByPath(sensorPath, valueType) {
        if(this.libreRunning) {
            if(valueType = "current") {
                valueType = "Value"
            } else if (valueType = "min") {
                valueType = "Min"
            } else if (valueType = "max") {
                valueType = "Max"
            } else {
                valueType = "Value"
            }
            let results = this.query("Select " + valueType + " From Sensor Where Identifier = \"" + sensorPath + "\"");
            return results[0][valueType];
        } else {
            return 0;
        }
    }

    returnUnits(sensorType) {
        const unitsObject = {
            Voltage: "V",
            Current: "A",
            Power: "W",
            Clock: "MHz",
            Temperature: "°C",
            Load: "%",
            Frequency: "Hz",
            Fan: "RPM",
            Flow: "L/h",
            Control: "%",
            Level: "%",
            Factor: "",
            Data: "GB",
            SmallData: "MB",
            Throughput: "B/s",
            TimeSpan: "Seconds", 
            Energy: "mWh",
            Noise: "dBA"
        };
        return unitsObject[sensorType];
    }
}

module.exports = Sensors;