using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Text;
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

namespace LibreHardwareMonitorNative
{
    public static class LibreHardwareMonitorNative
    {
        // Storage for computer objects. Since throwing around managed objects
        // is fairly expensive, we store them in a dictionary and pass around
        // the hash code instead. Integers are super-cheap to pass around.
        private static Dictionary<int, Computer> computers = new Dictionary<int, Computer>();

        [UnmanagedCallersOnly(EntryPoint = "create_computer_object")]
        public static int CreateComputerObject()
        {
            Computer computer = new Computer();
            computer.Open();
            computer.Accept(new UpdateVisitor());
            computers.Add(computer.GetHashCode(), computer);
            return computer.GetHashCode();
        }

        private static IHardware find_hardware_by_indices(int computerId, IntPtr indicesArrPtr, int indicesLen)
        {
            int[] indices = new int[indicesLen];
            Marshal.Copy(indicesArrPtr, indices, 0, indicesLen);
            var hardware = computers[computerId].Hardware;
            for (int i = 0; i < (indicesLen - 1); i++)
            {
                hardware = hardware[indices[i]].SubHardware;
            }
            return hardware[indices[indicesLen - 1]];
        }

        private static ISensor find_sensor_by_indices(int computerId, IntPtr indicesArrPtr, int indicesLen, int sensorIndex)
        {
            var hardware = find_hardware_by_indices(computerId, indicesArrPtr, indicesLen);
            return hardware.Sensors[sensorIndex];
        }

        [UnmanagedCallersOnly(EntryPoint = "reset_max_sensor_value")]
        public static void ResetMaxSensorValue(int computerId, IntPtr indicesArrPtr, int indicesLen, int sensorIndex)
        {
            var sensor = find_sensor_by_indices(computerId, indicesArrPtr, indicesLen, sensorIndex);
            sensor.ResetMax();
        }

        [UnmanagedCallersOnly(EntryPoint = "reset_min_sensor_value")]
        public static void ResetMinSensorValue(int computerId, IntPtr indicesArrPtr, int indicesLen, int sensorIndex)
        {
            var sensor = find_sensor_by_indices(computerId, indicesArrPtr, indicesLen, sensorIndex);
            sensor.ResetMin();
        }

        [UnmanagedCallersOnly(EntryPoint = "clear_sensor_values")]
        public static void ClearSensorValues(int computerId, IntPtr indicesArrPtr, int indicesLen, int sensorIndex)
        {
            var sensor = find_sensor_by_indices(computerId, indicesArrPtr, indicesLen, sensorIndex);
            sensor.ClearValues();
        }

        [UnmanagedCallersOnly(EntryPoint = "get_max_value_sensor")]
        public static float GetMaxValueSensor(int computerId, IntPtr indicesArrPtr, int indicesLen, int sensorIndex)
        {
            var sensor = find_sensor_by_indices(computerId, indicesArrPtr, indicesLen, sensorIndex);
            return sensor.Max ?? -1;
        }

        [UnmanagedCallersOnly(EntryPoint = "get_min_value_sensor")]
        public static float GetMinValueSensor(int computerId, IntPtr indicesArrPtr, int indicesLen, int sensorIndex)
        {
            var sensor = find_sensor_by_indices(computerId, indicesArrPtr, indicesLen, sensorIndex);
            return sensor.Min ?? -1;
        }

        [UnmanagedCallersOnly(EntryPoint = "get_sensor_type")]
        public static SensorType GetSensorType(int computerId, IntPtr indicesArrPtr, int indicesLen, int sensorIndex)
        {
            var sensor = find_sensor_by_indices(computerId, indicesArrPtr, indicesLen, sensorIndex);
            return sensor.SensorType;
        }

        [UnmanagedCallersOnly(EntryPoint = "set_sensor_name")]
        public static int SetSensorName(int computerId, IntPtr indicesArrPtr, int indicesLen, int sensorIndex, IntPtr namePtr)
        {
            int returnVal = 0;

            var sensor = find_sensor_by_indices(computerId, indicesArrPtr, indicesLen, sensorIndex);
            string name = Marshal.PtrToStringUTF8(namePtr) ?? sensor.Name;
            if (namePtr != IntPtr.Zero)
            {
                sensor.Name = name;
            }
            else
            {
                returnVal = -1;
            }

            return returnVal;
        }

        [UnmanagedCallersOnly(EntryPoint = "get_sensor_name")]
        public static IntPtr GetSensorName(int computerId, IntPtr indicesArrPtr, int indicesLen, int sensorIndex)
        {
            var sensor = find_sensor_by_indices(computerId, indicesArrPtr, indicesLen, sensorIndex);
            string name = sensor.Name;
            byte[] bytes = Encoding.UTF8.GetBytes(name);
            IntPtr ptr = Marshal.AllocHGlobal(bytes.Length + 1);
            Marshal.Copy(bytes, 0, ptr, bytes.Length);
            Marshal.WriteByte(ptr, bytes.Length, 0);
            return ptr;
        }

        [UnmanagedCallersOnly(EntryPoint = "get_sensor_value")]
        public static float GetSensorValue(int computerId, IntPtr indicesArrPtr, int indicesLen, int sensorIndex)
        {
            var sensor = find_sensor_by_indices(computerId, indicesArrPtr, indicesLen, sensorIndex);
            return sensor.Value ?? -1;
        }

        [UnmanagedCallersOnly(EntryPoint = "get_hardware_report")]
        public static IntPtr GetHardwareReport(int computerId, IntPtr indicesArrPtr, int indicesLen)
        {
            var hardware = find_hardware_by_indices(computerId, indicesArrPtr, indicesLen);
            string report = hardware.GetReport();
            byte[] bytes = Encoding.UTF8.GetBytes(report);
            IntPtr ptr = Marshal.AllocHGlobal(bytes.Length + 1);
            Marshal.Copy(bytes, 0, ptr, bytes.Length);
            Marshal.WriteByte(ptr, bytes.Length, 0);
            return ptr;
        }

        [UnmanagedCallersOnly(EntryPoint = "update_hardware_object")]
        public static void UpdateHardwareObject(int computerId, IntPtr indicesArrPtr, int indicesLen)
        {
            var hardware = find_hardware_by_indices(computerId, indicesArrPtr, indicesLen);
            hardware.Update();
        }

        [UnmanagedCallersOnly(EntryPoint = "get_subhardware_len_hardware")]
        public static int GetSubhardwareLenHardware(int computerId, IntPtr indicesArrPtr, int indicesLen)
        {
            var hardware = find_hardware_by_indices(computerId, indicesArrPtr, indicesLen);
            return hardware.SubHardware.Length;
        }

        [UnmanagedCallersOnly(EntryPoint = "get_sensors_len_hardware")]
        public static int GetSensorsLenHardware(int computerId, IntPtr indicesArrPtr, int indicesLen)
        {
            var hardware = find_hardware_by_indices(computerId, indicesArrPtr, indicesLen);
            return hardware.Sensors.Length;
        }


        [UnmanagedCallersOnly(EntryPoint = "set_hardware_name")]
        public static int SetHardwareName(int computerId, IntPtr indicesArrPtr, int indicesLen, IntPtr namePtr)
        {
            int returnVal = 0;

            var hardware = find_hardware_by_indices(computerId, indicesArrPtr, indicesLen);
            string name = Marshal.PtrToStringUTF8(namePtr) ?? hardware.Name;
            if (namePtr != IntPtr.Zero)
            {
                hardware.Name = name;
            }
            else
            {
                returnVal = -1;
            }

            return returnVal;
        }

        [UnmanagedCallersOnly(EntryPoint = "get_hardware_name")]
        public static IntPtr GetHardwareName(int computerId, IntPtr indicesArrPtr, int indicesLen)
        {
            try {
                var hardware = find_hardware_by_indices(computerId, indicesArrPtr, indicesLen);
                string name = hardware.Name;
                byte[] bytes = Encoding.UTF8.GetBytes(name);
                IntPtr ptr = Marshal.AllocHGlobal(bytes.Length + 1);
                Marshal.Copy(bytes, 0, ptr, bytes.Length);
                Marshal.WriteByte(ptr, bytes.Length, 0);
                return ptr;
            }
            catch {
                return IntPtr.Zero;
            }
        }

        [UnmanagedCallersOnly(EntryPoint = "get_hardware_type")]
        public static HardwareType GetHardwareType(int computerId, IntPtr indicesArrPtr, int indicesLen)
        {
            var hardware = find_hardware_by_indices(computerId, indicesArrPtr, indicesLen);
            return hardware.HardwareType;
        }

        [UnmanagedCallersOnly(EntryPoint = "get_computer_hardware_len")]
        public static int GetComputerHardwareLen(int computerId)
        {
            return computers[computerId].Hardware.Count;
        }

        [UnmanagedCallersOnly(EntryPoint = "get_computer_report")]
        public static IntPtr GetComputerReport(int computerId) {
            string report = computers[computerId].GetReport();
            byte[] bytes = Encoding.UTF8.GetBytes(report);
            IntPtr ptr = Marshal.AllocHGlobal(bytes.Length + 1);
            Marshal.Copy(bytes, 0, ptr, bytes.Length);
            Marshal.WriteByte(ptr, bytes.Length, 0);
            return ptr;
        }

        [UnmanagedCallersOnly(EntryPoint = "free_dotnet_string")]  
        public static void FreeDotnetString(IntPtr ptr) {
            Marshal.FreeHGlobal(ptr);
        }

        [UnmanagedCallersOnly(EntryPoint = "destroy_computer_object")]
        public static void DestroyComputerObject(int computerId)
        {
            computers[computerId].Close();
            computers.Remove(computerId);
        }

        [UnmanagedCallersOnly(EntryPoint = "reset_computer_object")]
        public static void ResetComputerObject(int computerId)
        {
            computers[computerId].Reset();
        }

        [UnmanagedCallersOnly(EntryPoint = "update_computer_object")]
        public static void UpdateComputerObject(int computerId)
        {
            computers[computerId].Accept(new UpdateVisitor());
        }

        [UnmanagedCallersOnly(EntryPoint = "set_computer_is_network_enabled")]
        public static void SetComputerIsNetworkEnabled(int computerId, bool enabled)
        {
            computers[computerId].IsNetworkEnabled = enabled;
        }

        [UnmanagedCallersOnly(EntryPoint = "get_computer_is_network_enabled")]
        public static bool GetComputerIsNetworkEnabled(int computerId)
        {
            return computers[computerId].IsNetworkEnabled;
        }

        [UnmanagedCallersOnly(EntryPoint = "set_computer_is_battery_enabled")]
        public static void SetComputerIsBatteryEnabled(int computerId, bool enabled)
        {
            computers[computerId].IsBatteryEnabled = enabled;
        }

        [UnmanagedCallersOnly(EntryPoint = "get_computer_is_battery_enabled")]
        public static bool GetComputerIsBatteryEnabled(int computerId)
        {
            return computers[computerId].IsBatteryEnabled;
        }

        [UnmanagedCallersOnly(EntryPoint = "set_computer_is_cpu_enabled")]
        public static void SetComputerIsCpuEnabled(int computerId, bool enabled)
        {
            computers[computerId].IsCpuEnabled = enabled;
        }

        [UnmanagedCallersOnly(EntryPoint = "get_computer_is_cpu_enabled")]
        public static bool GetComputerIsCpuEnabled(int computerId)
        {
            return computers[computerId].IsCpuEnabled;
        }

        [UnmanagedCallersOnly(EntryPoint = "set_computer_is_gpu_enabled")]
        public static void SetComputerIsGpuEnabled(int computerId, bool enabled)
        {
            computers[computerId].IsGpuEnabled = enabled;
        }

        [UnmanagedCallersOnly(EntryPoint = "get_computer_is_gpu_enabled")]
        public static bool GetComputerIsGpuEnabled(int computerId)
        {
            return computers[computerId].IsGpuEnabled;
        }

        [UnmanagedCallersOnly(EntryPoint = "set_computer_is_memory_enabled")]
        public static void SetComputerIsMemoryEnabled(int computerId, bool enabled)
        {
            computers[computerId].IsMemoryEnabled = enabled;
        }

        [UnmanagedCallersOnly(EntryPoint = "get_computer_is_memory_enabled")]
        public static bool GetComputerIsMemoryEnabled(int computerId)
        {
            return computers[computerId].IsMemoryEnabled;
        }

        [UnmanagedCallersOnly(EntryPoint = "set_computer_is_motherboard_enabled")]
        public static void SetComputerIsMotherboardEnabled(int computerId, bool enabled)
        {
            computers[computerId].IsMotherboardEnabled = enabled;
        }

        [UnmanagedCallersOnly(EntryPoint = "get_computer_is_motherboard_enabled")]
        public static bool GetComputerIsMotherboardEnabled(int computerId)
        {
            return computers[computerId].IsMotherboardEnabled;
        }

        [UnmanagedCallersOnly(EntryPoint = "set_computer_is_storage_enabled")]
        public static void SetComputerIsStorageEnabled(int computerId, bool enabled)
        {
            computers[computerId].IsStorageEnabled = enabled;
        }

        [UnmanagedCallersOnly(EntryPoint = "get_computer_is_storage_enabled")]
        public static bool GetComputerIsStorageEnabled(int computerId)
        {
            return computers[computerId].IsStorageEnabled;
        }

        [UnmanagedCallersOnly(EntryPoint = "set_computer_is_controller_enabled")]
        public static void SetComputerIsControllerEnabled(int computerId, bool enabled)
        {
            computers[computerId].IsControllerEnabled = enabled;
        }

        [UnmanagedCallersOnly(EntryPoint = "get_computer_is_controller_enabled")]
        public static bool GetComputerIsControllerEnabled(int computerId)
        {
            return computers[computerId].IsControllerEnabled;
        }

        [UnmanagedCallersOnly(EntryPoint = "set_computer_is_psu_enabled")]
        public static void SetComputerIsPsuEnabled(int computerId, bool enabled)
        {
            computers[computerId].IsPsuEnabled = enabled;
        }

        [UnmanagedCallersOnly(EntryPoint = "get_computer_is_psu_enabled")]
        public static bool GetComputerIsPsuEnabled(int computerId)
        {
            return computers[computerId].IsPsuEnabled;
        }
    }
}