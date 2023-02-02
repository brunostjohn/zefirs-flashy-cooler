using LibreHardwareMonitor.Hardware;
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
        Console.WriteLine("\t\"" + hardware.Name + "\": {");
        if (hardware.SubHardware.Any())
        {
            Console.WriteLine("\t\t\"subdevices\": {");
        }
        foreach (IHardware subhardware in hardware.SubHardware)
        {
            Console.WriteLine("\t\t\t\""+subhardware.Name+"\": {");

            foreach (ISensor sensor in subhardware.Sensors)
            {
                Console.WriteLine("\t\t\t\t\"{0}\": \"{1}\"", sensor.Name, sensor.Value);
            }
                if (subhardware.Equals(hardware.SubHardware.Last()))
                {
                Console.WriteLine("\t\t\t}");
                } else
                {
                    Console.WriteLine("\t\t\t},");
                }

            Console.WriteLine("\t\t}");
        }
        if (hardware.SubHardware.Any())
        {
            Console.WriteLine("\t}");
        }
        Console.WriteLine("\"sensors\": {");
        foreach (ISensor sensor in hardware.Sensors)
        {
            Console.WriteLine("\"{0}\": \"{1}\"", sensor.Name, sensor.Value);
        }
        if (hardware.Equals(computer.Hardware.Last()))
        {
        Console.WriteLine("}");
        } else
        {
            Console.WriteLine("},");
        }
    }
    Console.WriteLine("}");
    computer.Close();
}

Monitor();
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
