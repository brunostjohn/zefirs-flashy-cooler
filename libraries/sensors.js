require("winax");

class Sensors {
    constructor() {
        this.connection = new ActiveXObject("WbemScripting.SWbemLocator");
        this.svr = this.connection.ConnectServer(".", "root\\LibreHardwareMonitor");
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

    getHardware() {
        return this.query('Select * from Hardware');
    }

    getGPUTemps() {
        const temps = [];
        const sensorResults = this.svr.ExecQuery('Select * from Sensor Where (Parent LIKE "/nvidiagpu/[0-9]" OR Parent LIKE "/atigpu/[0-9]") AND SensorType = "Temperature"');
        console.log(sensorResults);
        for (let i = 0; i < sensorResults.Count; i += 1) {
          const p = sensorResults.ItemIndex(i).Properties_;
          temps.push({
            Name: p.Item('Name').Value,
            Identifier: p.Item('Identifier').Value,
            InstanceId: p.Item('InstanceId').Value,
            Min: p.Item('Min').Value,
            Max: p.Item('Max').Value,
          });
        }
    
        return temps;
    }
}

module.exports = Sensors;