using System;
using System.Runtime.InteropServices;

namespace Kcm
{
    /// <summary>
    /// KCM Knowledge Columnar Model - .NET SDK
    /// P/Invoke bindings to the KCM C FFI.
    /// </summary>
    public class KcmDatabase : IDisposable
    {
        private IntPtr _handle;
        private bool _disposed;

        public KcmDatabase()
        {
            _handle = NativeMethods.KCM_DatabaseNew();
            if (_handle == IntPtr.Zero)
                throw new KcmException("Failed to create database");
        }

        public void Insert(Fact fact)
        {
            ObjectDisposedException.ThrowIf(_disposed, this);
            var cf = fact.ToNative();
            NativeMethods.KCM_DatabaseInsert(_handle, ref cf);
        }

        public void Delete(ulong rowId)
        {
            ObjectDisposedException.ThrowIf(_disposed, this);
            NativeMethods.KCM_DatabaseDelete(_handle, rowId);
        }

        public ulong FactCount()
        {
            ObjectDisposedException.ThrowIf(_disposed, this);
            return NativeMethods.KCM_DatabaseFactCount(_handle);
        }

        public ulong ActiveFactCount()
        {
            ObjectDisposedException.ThrowIf(_disposed, this);
            return NativeMethods.KCM_DatabaseActiveCount(_handle);
        }

        public void Dispose()
        {
            if (!_disposed && _handle != IntPtr.Zero)
            {
                NativeMethods.KCM_DatabaseFree(_handle);
                _handle = IntPtr.Zero;
                _disposed = true;
            }
            GC.SuppressFinalize(this);
        }

        ~KcmDatabase() => Dispose();
    }

    public class Fact
    {
        public uint Subject { get; set; }
        public byte Predicate { get; set; }
        public uint Object { get; set; }
        public double Confidence { get; set; }

        public Fact(uint subject, byte predicate, @uint obj, double confidence)
        {
            if (confidence < 0.0 || confidence > 1.0)
                throw new ArgumentException($"Confidence must be in [0, 1], got {confidence}");
            Subject = subject;
            Predicate = predicate;
            Object = obj;
            Confidence = confidence;
        }

        internal KCM_Fact ToNative() => new()
        {
            subject = Subject,
            predicate = Predicate,
            @object = Object,
            confidence = Confidence
        };

        public override string ToString() =>
            $"Fact{{Subject={Subject}, Predicate={Predicate}, Object={Object}, Confidence={Confidence:F2}}}";
    }

    public class KcmException : Exception
    {
        public KcmException(string message) : base(message) { }
    }

    internal static class NativeMethods
    {
        private const string Lib = "kcm";

        [DllImport(Lib)] internal static extern IntPtr KCM_DatabaseNew();
        [DllImport(Lib)] internal static extern void KCM_DatabaseFree(IntPtr db);
        [DllImport(Lib)] internal static extern void KCM_DatabaseInsert(IntPtr db, ref KCM_Fact fact);
        [DllImport(Lib)] internal static extern void KCM_DatabaseDelete(IntPtr db, ulong rowId);
        [DllImport(Lib)] internal static extern ulong KCM_DatabaseFactCount(IntPtr db);
        [DllImport(Lib)] internal static extern ulong KCM_DatabaseActiveCount(IntPtr db);
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct KCM_Fact
    {
        public uint subject;
        public byte predicate;
        public uint @object;
        public double confidence;
    }
}
