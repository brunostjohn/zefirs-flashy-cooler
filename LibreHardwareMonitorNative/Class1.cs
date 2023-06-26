using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using LibreHardwareMonitor;
using LibreHardwareMonitor.Hardware;

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

[StructLayout(LayoutKind.Sequential)]
public struct PreSensor
{
    public IntPtr sensor, value, type, parent_hw_type;
}

namespace LibreHardwareMonitorNative
{

    public class LibreHardwareMonitorNative
    {
        [UnmanagedCallersOnly(EntryPoint = "free_mem")]
        public static void Free(IntPtr ptr)
        {
            Marshal.FreeHGlobal(ptr);
        }

        [UnmanagedCallersOnly(EntryPoint = "open_computer")]
        public static IntPtr OpenComputer()
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

            GCHandle gch = GCHandle.Alloc(computer, GCHandleType.Normal);
            gch.Target = computer;
            IntPtr objectPtr = GCHandle.ToIntPtr(gch);
            return objectPtr;
        }

        [UnmanagedCallersOnly(EntryPoint = "close_computer")]
        public static void CloseComputer(IntPtr gchPtr)
        {
            GCHandle gch = GCHandle.FromIntPtr(gchPtr);
            object ob = gch.Target;
            Computer computer = (Computer)ob;
            computer.Close();
            gch.Free();
        }

        [UnmanagedCallersOnly(EntryPoint = "get_single_sensor_ptrs")]
        public static PreSensor GetSingleSensor(IntPtr sensorPathString, IntPtr gchPtr)
        {
            GCHandle gch = GCHandle.FromIntPtr(gchPtr);
            object ob = gch.Target;
            Computer computer = (Computer)ob;

            computer.Accept(new UpdateVisitor());
            //if (sensorPathString == IntPtr.Zero) return Marshal.StringToHGlobalAnsi("failed");
            string sensor = Marshal.PtrToStringAnsi(sensorPathString) ?? "failed";

            if (sensor == "failed" || sensor.Length < 1)
            {
                PreSensor alter = new PreSensor();
                alter.sensor = Marshal.StringToHGlobalAnsi("a");
                alter.value = Marshal.StringToHGlobalAnsi("a");
                alter.type = Marshal.StringToHGlobalAnsi("a");
                alter.parent_hw_type = Marshal.StringToHGlobalAnsi("a");

                return alter;
            }

            if (sensor != "failed")
            {
                string[] splitPath = sensor.Split("/");

                IHardware parent = computer.Hardware.FirstOrDefault(x => x.Name == splitPath[0]) ?? computer.Hardware.First();
                if (parent == null)
                {
                    // return Marshal.StringToHGlobalAnsi("failed");
                }

                ISensor sensorClass;
                string hwType = "";

                if (splitPath[1] == "subhardware")
                {
                    IHardware sub = parent.SubHardware.FirstOrDefault(x => x.Name == splitPath[2]) ?? parent.SubHardware.First();
                    hwType = sub.HardwareType.ToString();
                    sensorClass = sub.Sensors.Where(x => x.Name == splitPath[4] && x.SensorType.ToString() == splitPath[3]).FirstOrDefault() ?? sub.Sensors.First();
                }
                else
                {
                    sensorClass = parent.Sensors.FirstOrDefault(x => x.Name == splitPath[2] && x.SensorType.ToString() == splitPath[1]) ?? parent.Sensors.First();
                    hwType = parent.HardwareType.ToString();
                }

                string serialised = "{\"sensor\": \"" + sensorClass.Name + "\",\"value\": \"" + sensorClass.Value + "\",\"type\": \"" + sensorClass.SensorType + "\", \"parent_hw_type\": \"" + hwType + "\"}";

                PreSensor alter = new PreSensor();
                alter.sensor = Marshal.StringToHGlobalAnsi(sensorClass.Name);
                alter.value = Marshal.StringToHGlobalAnsi(sensorClass.Value.ToString());
                alter.type = Marshal.StringToHGlobalAnsi(sensorClass.SensorType.ToString());
                alter.parent_hw_type = Marshal.StringToHGlobalAnsi(hwType);

                return alter;

            }

            //return Marshal.StringToHGlobalAnsi("failed");
            return new PreSensor();
        }

        [UnmanagedCallersOnly(EntryPoint = "get_all_sensors")]
        public static IntPtr GetAllSensors(IntPtr gchPtr)
        {
            GCHandle gch = GCHandle.FromIntPtr(gchPtr);
            object ob = gch.Target;
            Computer computer = (Computer)ob;
            string serialised = "[";

            computer.Accept(new UpdateVisitor());

            foreach (IHardware hardware in computer.Hardware)
            {
                serialised += "{\"name\": \"" + hardware.Name + "\",";

                if (hardware.SubHardware.Length > 0)
                {
                    serialised += "\"subhardware\": [";
                }

                foreach (IHardware subhardware in hardware.SubHardware)
                {
                    serialised += "{\"name\": \"" + subhardware.Name + "\",\"sensors\": [";

                    foreach (ISensor sensor in subhardware.Sensors)
                    {
                        serialised += "{";
                        if (sensor == subhardware.Sensors.Last())
                        {
                            serialised += "\"sensor\": \"" + sensor.Name + "\",\"value\": \"" + sensor.Value + "\",\"type\": \"" + sensor.SensorType + "\"}";
                        }
                        else
                        {
                            serialised += "\"sensor\": \"" + sensor.Name + "\",\"value\": \"" + sensor.Value + "\",\"type\": \"" + sensor.SensorType + "\"},";
                        }
                    }

                    if (subhardware == hardware.SubHardware.Last())
                    {
                        serialised += "]";
                    }
                    else
                    {
                        serialised += "],";
                    }
                    serialised += "}";
                }

                if (hardware.SubHardware.Length > 0 && hardware.Sensors.Length < 1)
                {
                    serialised += "],";
                }
                else if (hardware.SubHardware.Length > 0 && hardware.Sensors.Length > 0)
                {
                    serialised += "],";
                }

                serialised += "\"sensors\": [";
                foreach (ISensor sensor in hardware.Sensors)
                {
                    serialised += "{";
                    if (sensor == hardware.Sensors.Last())
                    {
                        serialised += "\"sensor\": \"" + sensor.Name + "\",\"value\": \"" + sensor.Value + "\",\"type\": \"" + sensor.SensorType + "\"}";
                    }
                    else
                    {
                        serialised += "\"sensor\": \"" + sensor.Name + "\",\"value\": \"" + sensor.Value + "\",\"type\": \"" + sensor.SensorType + "\"},";
                    }
                }
                serialised += "]";

                if (hardware == computer.Hardware.Last())
                {
                    serialised += "}";
                }
                else
                {
                    serialised += "},";
                }
            }
            serialised += "]";

            return Marshal.StringToHGlobalAnsi(serialised);
        }
    }
}