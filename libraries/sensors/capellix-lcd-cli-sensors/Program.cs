using LibreHardwareMonitor.Hardware;

static string generateID()
{
    //return Guid.NewGuid().ToString("N");
    return "";
}
static void Monitor()
{
    Computer computer = new Computer
    {
        IsCpuEnabled = true,
        IsGpuEnabled = true,
        IsMemoryEnabled = true,
        IsMotherboardEnabled = true,
        IsControllerEnabled = true,
        IsNetworkEnabled = true,
        IsStorageEnabled = true
    };

    computer.Open();
    computer.Accept(new UpdateVisitor());
    Console.WriteLine("{");
    foreach (IHardware hardware in computer.Hardware)
    {
        Console.WriteLine("\t\"" + hardware.Name + generateID() + "\": {");
        foreach (IHardware subhardware in hardware.SubHardware)
        {
            Console.WriteLine("\t\t\""+subhardware.Name+ generateID() +"\": {");

            String subhardwarenamearray = "[";
            String subhardwarevaluearray = "[";

            foreach (ISensor sensor in subhardware.Sensors)
            {
                if (sensor.Equals(subhardware.Sensors.Last()))
                {
                    subhardwarenamearray += "\"" + sensor.Name + "\"]";
                    Console.WriteLine("\t\t\t\"names\": " + subhardwarenamearray + ",");
                    subhardwarevaluearray += "\"" + sensor.Value + "\"]";
                    Console.WriteLine("\t\t\t\"values\": " + subhardwarevaluearray);
                    subhardwarenamearray = "[";
                    subhardwarevaluearray = "[";
                } else
                {
                    subhardwarenamearray += "\"" + sensor.Name + "\",";
                    subhardwarevaluearray += "\"" + sensor.Value + "\",";
                }
            }
                if (subhardware.Equals(hardware.SubHardware.Last()))
                {
                Console.WriteLine("\t\t},");
                } else
                {
                    Console.WriteLine("\t\t},");
                }
        }

        String hardwarenamearray = "[";
        String hardwarevaluearray = "[";
        Console.WriteLine("\t\t\"Main Sensors\": {");
        foreach (ISensor sensor in hardware.Sensors)
        {
            if (sensor.Equals(hardware.Sensors.Last()))
            {
                hardwarenamearray += "\"" + sensor.Name + "\"]";
                Console.WriteLine("\t\t\t\"names\": " + hardwarenamearray + ",");
                hardwarevaluearray += "\"" + sensor.Value + "\"]";
                Console.WriteLine("\t\t\t\"values\": " + hardwarevaluearray);
                hardwarenamearray = "[";
                hardwarevaluearray = "[";
            }
            else
            {
                hardwarenamearray += "\"" + sensor.Name + "\",";
                hardwarevaluearray += "\"" + sensor.Value + "\",";
            }
        }
        Console.WriteLine("\t\t}");
        if (hardware.Equals(computer.Hardware.Last()))
        {
            Console.WriteLine("\t}");
        } else
        {
            Console.WriteLine("\t},");
        }
    }
    Console.WriteLine("}");
    computer.Close();
}

Monitor();
Environment.Exit(0);
public class UpdateVisitor : IVisitor
{
    public void VisitComputer(IComputer computer)
    {
        computer.Traverse(this);
    }
    public void VisitHardware(IHardware hardware)
    {
        hardware.Update();
        foreach (IHardware subHardware in hardware.SubHardware) subHardware.Accept(this);
    }
    public void VisitSensor(ISensor sensor) { }
    public void VisitParameter(IParameter parameter) { }
}
