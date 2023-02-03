const {execFile, execFileSync} = require("child_process");
const path = require("path");

class Sensors {
    constructor(persistentUpdate=true) {
        this.sensordata = this.updateSensorsSyncAlt();
        this.firstrun = true;
        if(persistentUpdate == true) {
            this.startUpdating();
        } else {
            this.updateSensorsSync();
        }
    }

    updateSensors() {
        execFile(path.join(__dirname, "sensors", "capellix-lcd-cli-sensors", "bin", "Release", "net6.0", "capellix-lcd-cli-sensors.exe"), (error, stdout, stderr) => {
            this.sensordata = JSON.parse(stdout);
        });
    }

    updateSensorsSync() {
        this.sensordata = JSON.parse(execFileSync(path.join(__dirname, "sensors", "capellix-lcd-cli-sensors", "bin", "Release", "net6.0", "capellix-lcd-cli-sensors.exe")));
    }

    updateSensorsSyncAlt() {
        this.sensordata = JSON.parse(execFileSync(path.join(__dirname, "sensors", "capellix-lcd-cli-sensors", "bin", "Release", "net6.0", "capellix-lcd-cli-sensors.exe")));
    }

    startUpdating() {
        this.timer = setInterval(this.updateSensors, 500);
    }

    stopUpdating() {
        clearTimeout(this.updateSensors);
    }

    fetchSensors() {
        return this.sensordata;
    }
}

module.exports = Sensors;