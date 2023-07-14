﻿using System.Runtime.CompilerServices;
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

[StructLayout(LayoutKind.Explicit, Size = 32)]
public struct Subscription
{
    [FieldOffset(0)]
    [MarshalAs(UnmanagedType.LPUTF8Str, SizeConst = 16)]
    public string code_name;

    [FieldOffset(16)]
    [MarshalAs(UnmanagedType.LPUTF8Str, SizeConst = 16)]
    public string path;
}

public struct SensorContainer
{
    public IHardware parent;
    public ISensor sensor;
}

namespace LibreHardwareMonitorNative
{
    public static class LibreHardwareMonitorNative
    {
        private static List<SensorContainer> sensorCache = new List<SensorContainer>();
        private static Computer computer = new Computer
        {
            IsCpuEnabled = true,
            IsGpuEnabled = true,
            IsMemoryEnabled = true,
            IsMotherboardEnabled = true,
            IsControllerEnabled = true,
            IsNetworkEnabled = true,
            IsStorageEnabled = true
        };

        [UnmanagedCallersOnly(EntryPoint = "free_mem")]
        public static void Free(IntPtr ptr)
        {
            Marshal.FreeHGlobal(ptr);
        }

        [UnmanagedCallersOnly(EntryPoint = "open_computer")]
        public static void OpenComputer()
        {
            computer.Open();
            computer.Accept(new UpdateVisitor());
        }

        [UnmanagedCallersOnly(EntryPoint = "close_computer")]
        public static void CloseComputer()
        {
            computer.Close();
        }

        [UnmanagedCallersOnly(EntryPoint = "subscribe")]
        public static IntPtr Subscribe(IntPtr inStructs, uint count)
        {
            Subscription[] subs = new Subscription[count];

            string subscribed = "";

            for (int i = 0; i < count; i++)
            {
                subs[i] = Marshal.PtrToStructure<Subscription>(inStructs + i * Marshal.SizeOf<Subscription>());
                string s_value = GetSingleSensor(subs[i].path);
                subscribed += s_value + "****";

                string[] splitPath = subs[i].path.Split("/");

                SensorContainer cached = new SensorContainer();

                IHardware parent = computer.Hardware.Where(x => x.Name == splitPath[0]).FirstOrDefault() ?? computer.Hardware.First();

                ISensor sensorClass;

                if (splitPath[1] == "subhardware")
                {
                    IHardware sub = parent.SubHardware.Where(x => x.Name == splitPath[2]).FirstOrDefault() ?? parent.SubHardware.First();
                    sub.Update();
                    cached.parent = sub;
                    sensorClass = sub.Sensors.Where(x => x.Name == splitPath[4] && x.SensorType.ToString() == splitPath[3]).FirstOrDefault() ?? sub.Sensors.First();
                    cached.sensor = sensorClass;
                }
                else
                {
                    parent.Update();
                    cached.parent = parent;
                    sensorClass = parent.Sensors.Where(x => x.Name == splitPath[2] && x.SensorType.ToString() == splitPath[1]).FirstOrDefault() ?? parent.Sensors.First();
                    cached.sensor = sensorClass;
                }

                sensorCache.Add(cached);
            }

            subscribed.Remove(subscribed.Length - 5);

            return Marshal.StringToHGlobalAnsi(subscribed);
        }

        [UnmanagedCallersOnly(EntryPoint = "get_subscribed_ptr")]
        public static IntPtr GetSubscribedPtr()
        {

            string finalSerialised = "";

            foreach (SensorContainer sensor in sensorCache)
            {
                sensor.parent.Update();
                finalSerialised += sensor.sensor.Value.ToString() + "||";
            }

            if (finalSerialised.Length == 0)
            {
                return Marshal.StringToHGlobalAnsi("FAILEDFAILEDFAILED");
            }

            finalSerialised.Remove(finalSerialised.Length - 3);

            return Marshal.StringToHGlobalAnsi(finalSerialised);

        }

        public static string GetSingleSensor(string sensorPathString)
        {

            string[] splitPath = sensorPathString.Split("/");

            IHardware parent = computer.Hardware.Where(x => x.Name == splitPath[0]).FirstOrDefault() ?? computer.Hardware.First();

            ISensor sensorClass;
            string hwType = "";

            if (splitPath[1] == "subhardware")
            {
                IHardware sub = parent.SubHardware.Where(x => x.Name == splitPath[2]).FirstOrDefault() ?? parent.SubHardware.First();
                sub.Update();
                hwType = sub.HardwareType.ToString();
                sensorClass = sub.Sensors.Where(x => x.Name == splitPath[4] && x.SensorType.ToString() == splitPath[3]).FirstOrDefault() ?? sub.Sensors.First();
            }
            else
            {
                parent.Update();
                sensorClass = parent.Sensors.Where(x => x.Name == splitPath[2] && x.SensorType.ToString() == splitPath[1]).FirstOrDefault() ?? parent.Sensors.First();
                hwType = parent.HardwareType.ToString();
            }

            string serialised = sensorClass.Name + "||" + sensorClass.Value + "||" + sensorClass.SensorType + "||" + hwType;

            return serialised;
        }


        [UnmanagedCallersOnly(EntryPoint = "get_all_sensors")]
        public static IntPtr GetAllSensors()
        {
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