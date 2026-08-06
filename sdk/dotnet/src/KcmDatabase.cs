using System;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;

namespace Kcm
{
    public enum KcmError
    {
        Ok = 0,
        NotFound = 1,
        OutOfMemory = 2,
        InvalidArgument = 3,
        Io = 4,
        Corrupted = 5,
        Conflict = 6,
        TransactionAborted = 7,
    }

    public class KcmException : Exception
    {
        public KcmError ErrorCode { get; }

        public KcmException(KcmError errorCode)
            : base(GetErrorMessage(errorCode))
        {
            ErrorCode = errorCode;
        }

        public KcmException(string message) : base(message)
        {
            ErrorCode = KcmError.InvalidArgument;
        }

        private static string GetErrorMessage(KcmError err)
        {
            try
            {
                IntPtr ptr = NativeMethods.KCM_ErrorMessage(err);
                if (ptr != IntPtr.Zero)
                {
                    string? msg = Marshal.PtrToStringUTF8(ptr);
                    if (!string.IsNullOrEmpty(msg))
                        return msg;
                }
            }
            catch (DllNotFoundException) { }
            catch (EntryPointNotFoundException) { }
            return err.ToString();
        }
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct KCM_Fact
    {
        public uint subject;
        public byte predicate;
        public uint @object;
        public double confidence;
        public byte evidence;
        public long timestamp;
        public byte context;
        public int version;
        public sbyte priority;
        public ushort owner;
    }

    public class Fact
    {
        public uint Subject { get; set; }
        public byte Predicate { get; set; }
        public uint Object { get; set; }
        public double Confidence { get; set; }
        public byte Evidence { get; set; }
        public long Timestamp { get; set; }
        public byte Context { get; set; }
        public int Version { get; set; }
        public sbyte Priority { get; set; }
        public ushort Owner { get; set; }

        public Fact(
            uint subject,
            byte predicate,
            uint obj,
            double confidence,
            byte evidence = 0,
            long timestamp = 0,
            byte context = 0,
            int version = 0,
            sbyte priority = 0,
            ushort owner = 0)
        {
            if (double.IsNaN(confidence) || confidence < 0.0 || confidence > 1.0)
                throw new ArgumentException($"Confidence must be in [0, 1], got {confidence}");
            Subject = subject;
            Predicate = predicate;
            Object = obj;
            Confidence = confidence;
            Evidence = evidence;
            Timestamp = timestamp;
            Context = context;
            Version = version;
            Priority = priority;
            Owner = owner;
        }

        internal KCM_Fact ToNative() => new()
        {
            subject = Subject,
            predicate = Predicate,
            @object = Object,
            confidence = Confidence,
            evidence = Evidence,
            timestamp = Timestamp,
            context = Context,
            version = Version,
            priority = Priority,
            owner = Owner,
        };

        internal static Fact FromNative(KCM_Fact native) => new(
            native.subject,
            native.predicate,
            native.@object,
            native.confidence,
            native.evidence,
            native.timestamp,
            native.context,
            native.version,
            native.priority,
            native.owner);

        public override string ToString() =>
            $"Fact{{Subject={Subject}, Predicate={Predicate}, Object={Object}, Confidence={Confidence:F2}, " +
            $"Evidence={Evidence}, Timestamp={Timestamp}, Context={Context}, Version={Version}, " +
            $"Priority={Priority}, Owner={Owner}}}";
    }

    internal static class NativeMethods
    {
        private const string Lib = "kcm";

        [DllImport(Lib)]
        internal static extern KcmError KCM_DatabaseNew(out IntPtr dbOut);

        [DllImport(Lib)]
        internal static extern void KCM_DatabaseFree(IntPtr db);

        [DllImport(Lib)]
        internal static extern KcmError KCM_DatabaseInsert(IntPtr db, ref KCM_Fact fact);

        [DllImport(Lib)]
        internal static extern KcmError KCM_DatabaseUpdate(IntPtr db, ulong rowId, ref KCM_Fact fact);

        [DllImport(Lib)]
        internal static extern KcmError KCM_DatabaseDelete(IntPtr db, ulong rowId);

        [DllImport(Lib)]
        internal static extern ulong KCM_DatabaseFactCount(IntPtr db);

        [DllImport(Lib)]
        internal static extern ulong KCM_DatabaseActiveCount(IntPtr db);

        [DllImport(Lib)]
        internal static extern IntPtr KCM_DatabaseQuery(IntPtr db,
            [MarshalAs(UnmanagedType.LPUTF8Str)] string query);

        [DllImport(Lib)]
        internal static extern IntPtr KCM_QueryNext(IntPtr query);

        [DllImport(Lib)]
        internal static extern void KCM_QueryFree(IntPtr query);

        [DllImport(Lib)]
        internal static extern IntPtr KCM_DatabaseBeginTransaction(IntPtr db);

        [DllImport(Lib)]
        internal static extern KcmError KCM_TransactionCommit(IntPtr txn);

        [DllImport(Lib)]
        internal static extern KcmError KCM_TransactionRollback(IntPtr txn);

        [DllImport(Lib)]
        internal static extern void KCM_TransactionFree(IntPtr txn);

        [DllImport(Lib)]
        internal static extern KcmError KCM_DatabaseSave(IntPtr db,
            [MarshalAs(UnmanagedType.LPUTF8Str)] string path);

        [DllImport(Lib)]
        internal static extern KcmError KCM_DatabaseLoad(IntPtr db,
            [MarshalAs(UnmanagedType.LPUTF8Str)] string path);

        [DllImport(Lib)]
        internal static extern KcmError KCM_DatabaseVerify(
            [MarshalAs(UnmanagedType.LPUTF8Str)] string path);

        [DllImport(Lib)]
        internal static extern IntPtr KCM_ErrorMessage(KcmError err);
    }

    public class KcmDatabase : IDisposable
    {
        private IntPtr _handle;
        private bool _disposed;

        public KcmDatabase()
        {
            var err = NativeMethods.KCM_DatabaseNew(out _handle);
            if (err != KcmError.Ok)
                throw new KcmException(err);
            if (_handle == IntPtr.Zero)
                throw new KcmException("Failed to create database");
        }

        internal KcmDatabase(IntPtr handle)
        {
            _handle = handle;
        }

        public void Insert(Fact fact)
        {
            ObjectDisposedException.ThrowIf(_disposed, this);
            var native = fact.ToNative();
            var err = NativeMethods.KCM_DatabaseInsert(_handle, ref native);
            if (err != KcmError.Ok)
                throw new KcmException(err);
        }

        public void Update(ulong rowId, Fact fact)
        {
            ObjectDisposedException.ThrowIf(_disposed, this);
            var native = fact.ToNative();
            var err = NativeMethods.KCM_DatabaseUpdate(_handle, rowId, ref native);
            if (err != KcmError.Ok)
                throw new KcmException(err);
        }

        public void Delete(ulong rowId)
        {
            ObjectDisposedException.ThrowIf(_disposed, this);
            var err = NativeMethods.KCM_DatabaseDelete(_handle, rowId);
            if (err != KcmError.Ok)
                throw new KcmException(err);
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

        public KcmQuery Query(string kql)
        {
            ObjectDisposedException.ThrowIf(_disposed, this);
            IntPtr queryHandle = NativeMethods.KCM_DatabaseQuery(_handle, kql);
            if (queryHandle == IntPtr.Zero)
                throw new KcmException("Query failed");
            return new KcmQuery(queryHandle);
        }

        public KcmTransaction BeginTransaction()
        {
            ObjectDisposedException.ThrowIf(_disposed, this);
            IntPtr txnHandle = NativeMethods.KCM_DatabaseBeginTransaction(_handle);
            if (txnHandle == IntPtr.Zero)
                throw new KcmException("Failed to begin transaction");
            return new KcmTransaction(txnHandle);
        }

        public void Save(string path)
        {
            ObjectDisposedException.ThrowIf(_disposed, this);
            var err = NativeMethods.KCM_DatabaseSave(_handle, path);
            if (err != KcmError.Ok)
                throw new KcmException(err);
        }

        public void Load(string path)
        {
            ObjectDisposedException.ThrowIf(_disposed, this);
            var err = NativeMethods.KCM_DatabaseLoad(_handle, path);
            if (err != KcmError.Ok)
                throw new KcmException(err);
        }

        public static void Verify(string path)
        {
            var err = NativeMethods.KCM_DatabaseVerify(path);
            if (err != KcmError.Ok)
                throw new KcmException(err);
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

    public class KcmQuery : IEnumerable<Fact>, IDisposable
    {
        private IntPtr _handle;
        private bool _disposed;

        internal KcmQuery(IntPtr handle)
        {
            _handle = handle;
        }

        public IEnumerator<Fact> GetEnumerator()
        {
            ObjectDisposedException.ThrowIf(_disposed, this);

            while (true)
            {
                IntPtr factPtr = NativeMethods.KCM_QueryNext(_handle);
                if (factPtr == IntPtr.Zero)
                    yield break;

                var native = Marshal.PtrToStructure<KCM_Fact>(factPtr);
                yield return Fact.FromNative(native);
            }
        }

        IEnumerator IEnumerable.GetEnumerator() => GetEnumerator();

        public void Dispose()
        {
            if (!_disposed && _handle != IntPtr.Zero)
            {
                NativeMethods.KCM_QueryFree(_handle);
                _handle = IntPtr.Zero;
                _disposed = true;
            }
            GC.SuppressFinalize(this);
        }

        ~KcmQuery() => Dispose();
    }

    public class KcmTransaction : IDisposable
    {
        private IntPtr _handle;
        private bool _disposed;

        internal KcmTransaction(IntPtr handle)
        {
            _handle = handle;
        }

        public void Commit()
        {
            ObjectDisposedException.ThrowIf(_disposed, this);
            var err = NativeMethods.KCM_TransactionCommit(_handle);
            if (err != KcmError.Ok)
                throw new KcmException(err);
        }

        public void Rollback()
        {
            ObjectDisposedException.ThrowIf(_disposed, this);
            var err = NativeMethods.KCM_TransactionRollback(_handle);
            if (err != KcmError.Ok)
                throw new KcmException(err);
        }

        public void Dispose()
        {
            if (!_disposed && _handle != IntPtr.Zero)
            {
                NativeMethods.KCM_TransactionFree(_handle);
                _handle = IntPtr.Zero;
                _disposed = true;
            }
            GC.SuppressFinalize(this);
        }

        ~KcmTransaction() => Dispose();
    }
}
