// Copyright (c) William R. Fraser
// Licensed under the MIT License <LICENSE.md>
//! Extensible Storage Engine structures and types.

pub type JET_API_PTR = usize;
pub type JET_CBTYP = ::ULONG;
pub type JET_COLTYP = ::ULONG;
pub type JET_COLUMNID = ::ULONG;
pub type JET_DATESERIAL = f64;
pub type JET_DBID = ::ULONG;
pub type JET_ERR = ::LONG;
pub type JET_ERRCAT = ::INT; // actually a `typedef enum` in the C header.
pub type JET_GRBIT = ::ULONG;
pub type JET_HANDLE = JET_API_PTR;
pub type JET_INSTANCE = JET_API_PTR;
pub type JET_LS = JET_API_PTR;
pub type JET_OBJTYP = ::ULONG;
pub type JET_OSSNAPID = JET_API_PTR;
pub type JET_PCSTR = ::PCSTR;
pub type JET_PCWSTR = ::PCWSTR;
pub type JET_PSTR = ::PSTR;
pub type JET_PWSTR = ::PWSTR;
pub type JET_RELOP = ::INT; // actually a `typedef enum` in the C header.
pub type JET_SESID = JET_API_PTR;
pub type JET_SNP = ::ULONG;
pub type JET_SNT = ::ULONG;
pub type JET_TABLEID = JET_API_PTR;
pub type JET_PFNSTATUS = Option<unsafe extern "system" fn(
    JET_SESID, JET_SNP, JET_SNT, ::PVOID,
) -> JET_ERR>;
STRUCT!{struct JET_COLUMNCREATE_W {
    cbStruct: ::ULONG,
    szColumnName: ::PWSTR,
    coltyp: JET_COLTYP,
    cbMax: ::ULONG,
    grbit: JET_GRBIT,
    pvDefault: ::PVOID,
    cbDefault: ::ULONG,
    cp: ::ULONG,
    columnid: JET_COLUMNID,
    err: JET_ERR,
}}
STRUCT!{struct JET_COLUMNDEF {
    cbStruct: ::ULONG,
    columnid: JET_COLUMNID,
    coltyp: JET_COLTYP,
    wCountry: ::USHORT,
    langid: ::USHORT,
    cp: ::USHORT,
    wCollate: ::USHORT,
    cbMax: ::ULONG,
    grbit: JET_GRBIT,
}}
STRUCT!{struct JET_COMMIT_ID {
    signLog: JET_SIGNATURE,
    reserved: ::INT,
    commitId: ::INT64,
}}
STRUCT!{struct JET_CONDITIONALCOLUMN_W {
    cbStruct: ::ULONG,
    szColumnName: ::PWSTR,
    grbit: JET_GRBIT,
}}
STRUCT!{nodebug struct JET_ERRINFOBASIC_W{
    cbStruct: ::ULONG,
    errValue: JET_ERR,
    errcatMostSpecific: JET_ERRCAT,
    rgCategoricalHeirarchy: [u8; 8],
    lSourceLine: ::ULONG,
    rgszSourceFile: [::WCHAR; 64],
}}
STRUCT!{struct JET_INDEX_COLUMN {
    columnid: JET_COLUMNID,
    relop: JET_RELOP,
    pv: ::PVOID,
    cb: ::ULONG,
    grbit: JET_GRBIT,
}}
#[cfg(target_pointer_width = "32")]
STRUCT!{struct JET_INDEXID {
    cbStruct: ::ULONG,
    rgbIndexId: [::CHAR; 12],
}}
#[cfg(target_pointer_width = "64")]
STRUCT!{struct JET_INDEXID {
    cbStruct: ::ULONG,
    rgbIndexId: [::CHAR; 16],
}}
#[cfg(target_pointer_width = "32")]
STRUCT!{struct JET_INDEXCREATE_W{
    cbStruct: ::ULONG,
    szIndexName: ::PWSTR,
    szKey: ::PWSTR,
    cbKey: ::ULONG,
    grbit: JET_GRBIT,
    ulDensity: ::ULONG,
    u1: [u8; 4],
    u2: [u8; 4],
    rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_W,
    cConditionalColumn: ::ULONG,
    err: JET_ERR,
    cbKeyMost: ::ULONG,
}}
#[cfg(target_pointer_width = "64")]
STRUCT!{struct JET_INDEXCREATE_W{
    cbStruct: ::ULONG,
    szIndexName: ::PWSTR,
    szKey: ::PWSTR,
    cbKey: ::ULONG,
    grbit: JET_GRBIT,
    ulDensity: ::ULONG,
    u1: [u8; 8],
    u2: [u8; 8],
    rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_W,
    cConditionalColumn: ::ULONG,
    err: JET_ERR,
    cbKeyMost: ::ULONG,
}}
UNION!(JET_INDEXCREATE_W, u1, lcid, lcid_mut, ::ULONG);
UNION!(JET_INDEXCREATE_W, u1, pidxunicode, pidxunicode_mut, *mut JET_UNICODEINDEX);
UNION!(JET_INDEXCREATE_W, u2, cbVarSegMac, cbVarSegMac_mut, ::ULONG);
UNION!(JET_INDEXCREATE_W, u2, ptuplelimits, ptuplelimits_mut, *mut JET_TUPLELIMITS);
#[cfg(target_pointer_width = "32")]
STRUCT!{struct JET_INDEXCREATE2_W{
    cbStruct: ::ULONG,
    szIndexName: ::PWSTR,
    szKey: ::PWSTR,
    cbKey: ::ULONG,
    grbit: JET_GRBIT,
    ulDensity: ::ULONG,
    u1: [u8; 4],
    u2: [u8; 4],
    rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_W,
    cConditionalColumn: ::ULONG,
    err: JET_ERR,
    cbKeyMost: ::ULONG,
    pSpacehints: *mut JET_SPACEHINTS,
}}
#[cfg(target_pointer_width = "64")]
STRUCT!{struct JET_INDEXCREATE2_W{
    cbStruct: ::ULONG,
    szIndexName: ::PWSTR,
    szKey: ::PWSTR,
    cbKey: ::ULONG,
    grbit: JET_GRBIT,
    ulDensity: ::ULONG,
    u1: [u8; 8],
    u2: [u8; 8],
    rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_W,
    cConditionalColumn: ::ULONG,
    err: JET_ERR,
    cbKeyMost: ::ULONG,
    pSpacehints: *mut JET_SPACEHINTS,
}}
UNION!(JET_INDEXCREATE2_W, u1, lcid, lcid_mut, ::ULONG);
UNION!(JET_INDEXCREATE2_W, u1, pidxunicode, pidxunicode_mut, *mut JET_UNICODEINDEX);
UNION!(JET_INDEXCREATE2_W, u2, cbVarSegMac, cbVarSegMac_mut, ::ULONG);
UNION!(JET_INDEXCREATE2_W, u2, ptuplelimits, ptuplelimits_mut, *mut JET_TUPLELIMITS);
#[cfg(target_pointer_width = "32")]
STRUCT!{struct JET_INDEXCREATE3_W{
    cbStruct: ::ULONG,
    szIndexName: ::PWSTR,
    szKey: ::PWSTR,
    cbKey: ::ULONG,
    grbit: JET_GRBIT,
    ulDensity: ::ULONG,
    pidxunicode: *mut JET_UNICODEINDEX2,
    u1: [u8; 4],
    u2: [u8; 4],
    rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_W,
    cConditionalColumn: ::ULONG,
    err: JET_ERR,
    cbKeyMost: ::ULONG,
    pSpacehints: *mut JET_SPACEHINTS,
}}
#[cfg(target_pointer_width = "64")]
STRUCT!{struct JET_INDEXCREATE3_W{
    cbStruct: ::ULONG,
    szIndexName: ::PWSTR,
    szKey: ::PWSTR,
    cbKey: ::ULONG,
    grbit: JET_GRBIT,
    ulDensity: ::ULONG,
    pidxunicode: *mut JET_UNICODEINDEX2,
    u1: [u8; 8],
    u2: [u8; 8],
    rgconditionalcolumn: *mut JET_CONDITIONALCOLUMN_W,
    cConditionalColumn: ::ULONG,
    err: JET_ERR,
    cbKeyMost: ::ULONG,
    pSpacehints: *mut JET_SPACEHINTS,
}}
UNION!(JET_INDEXCREATE3_W, u1, lcid, lcid_mut, ::ULONG);
UNION!(JET_INDEXCREATE3_W, u1, pidxunicode, pidxunicode_mut, *mut JET_UNICODEINDEX);
UNION!(JET_INDEXCREATE3_W, u2, cbVarSegMac, cbVarSegMac_mut, ::ULONG);
UNION!(JET_INDEXCREATE3_W, u2, ptuplelimits, ptuplelimits_mut, *mut JET_TUPLELIMITS);
STRUCT!{#[repr(packed)] struct JET_LGPOS {
    ib: ::USHORT,
    isec: ::USHORT,
    lGeneration: ::LONG,
}}
STRUCT!{#[repr(packed)] struct JET_LOGTIME {
    bSeconds: u8,
    bMinutes: u8,
    bHours: u8,
    bDay: u8,
    bMonth: u8,
    bYear: u8,
    bFiller1: u8,
    bFiller2: u8,
}}
BITFIELD!(JET_LOGTIME bFiller1: u8 [
    fTimeIsUTC set_fTimeIsUTC [0..1],
    bMillisecondsLow set_bMillisecondsLow [1..8],
]);
BITFIELD!(JET_LOGTIME bFiller2: u8 [
    fReserved set_fReserved [0..1],
    bMillisecondsHigh set_bMillisecondsHigh [1..4],
    fUnused set_fUnused [4..8],
]);
STRUCT!{struct JET_RETINFO {
    cbStruct: ::ULONG,
    ibLongValue: ::ULONG,
    itagSequence: ::ULONG,
    columnidNextTagged: JET_COLUMNID,
}}
STRUCT!{struct JET_RETRIEVECOLUMN {
    columnid: JET_COLUMNID,
    pvData: ::PVOID,
    cbData: ::ULONG,
    cbActual: ::ULONG,
    grbit: JET_GRBIT,
    ibLongValue: ::ULONG,
    itagSequence: ::ULONG,
    columnidNextTagged: JET_COLUMNID,
    err: JET_ERR,
}}
STRUCT!{struct JET_RSTMAP_W {
    szDatabaseName: ::PWSTR,
    szNewDatabaseName: ::PWSTR,
}}
STRUCT!{nodebug struct JET_RSTINFO_W {
    cbStruct: ::ULONG,
    rgrstmap: *mut JET_RSTMAP_W,
    crstmap: ::LONG,
    lgposStop: JET_LGPOS,
    logtimeStop: JET_LOGTIME,
    pfnStatus: JET_PFNSTATUS,
}}
STRUCT!{struct JET_SETCOLUMN {
    columnid: JET_COLUMNID,
    pvdata: ::PVOID,
    cbdata: ::ULONG,
    grbit: JET_GRBIT,
    ibLongValue: ::ULONG,
    itagSequence: ::ULONG,
    err: JET_ERR,
}}
STRUCT!{struct JET_SETINFO {
    cbStruct: ::ULONG,
    ibLongValue: ::ULONG,
    itagSequence: ::ULONG,
}}
STRUCT!{struct JET_SIGNATURE {
    ulRandom: ::ULONG,
    logtimeCreate: JET_LOGTIME,
    szComputerName: [u8; JET_MAX_COMPUTERNAME_LENGTH + 1],
}}
STRUCT!{struct JET_SPACEHINTS {
    cbStruct: ::ULONG,
    ulInitialDensity: ::ULONG,
    cbInitial: ::ULONG,
    grbit: JET_GRBIT,
    ulMaintDensity: ::ULONG,
    ulGrowth: ::ULONG,
    cbMinExtent: ::ULONG,
    cbMaxExtent: ::ULONG,
}}
STRUCT!{struct JET_TABLECREATE_W {
    cbStruct: ::ULONG,
    szTableName: ::PWSTR,
    szTemplateTableName: ::PWSTR,
    ulPages: ::ULONG,
    ulDensity: ::ULONG,
    rgcolumncreate: *const JET_COLUMNCREATE_W,
    cColumns: ::ULONG,
    rcindexcreate: *const JET_INDEXCREATE_W,
    cIndexes: ::ULONG,
    grbit: JET_GRBIT,
    tableid: JET_TABLEID,
    cCreated: ::ULONG,
}}
STRUCT!{struct JET_TABLECREATE3_W {
    cbStruct: ::ULONG,
    szTableName: ::PWSTR,
    szTemplateTableName: ::PWSTR,
    ulPages: ::ULONG,
    ulDensity: ::ULONG,
    rgcolumncreate: *mut JET_COLUMNCREATE_W,
    cColumns: ::ULONG,
    rgindexcreate: *mut JET_INDEXCREATE_W,
    cIndexes: ::ULONG,
    grbit: JET_GRBIT,
    tableid: JET_TABLEID,
    cCreated: ::ULONG,
}}
STRUCT!{struct JET_TUPLELIMITS {
    chLengthMin: ::ULONG,
    chLengthMax: ::ULONG,
    chToIndexMax: ::ULONG,
    cchIncrement: ::ULONG,
    ichStart: ::ULONG,
}}
STRUCT!{struct JET_UNICODEINDEX {
    lcid: ::ULONG,
    dwMapFlags: ::ULONG,
}}
STRUCT!{struct JET_UNICODEINDEX2 {
    szLocaleName: ::PWSTR,
    dwMapFlags: ::ULONG,
}}
pub const JET_MAX_COMPUTERNAME_LENGTH: usize = 15;
pub const JET_ErrorInfoSpecificErr: ::ULONG = 1;
pub const JET_relopEquals: JET_RELOP = 0;
pub const JET_relopPrefixEquals: JET_RELOP = 1;
pub const JET_relopNotEquals: JET_RELOP = 2;
pub const JET_relopLessThanOrEqual: JET_RELOP = 3;
pub const JET_relopLessThan: JET_RELOP = 4;
pub const JET_relopGreaterThanOrEqual: JET_RELOP = 5;
pub const JET_relopGreaterThan: JET_RELOP = 6;
pub const JET_relopBitmaskEqualsZero: JET_RELOP = 7;
pub const JET_relopBitmaskNotEqualsZero: JET_RELOP = 8;
pub const JET_errcatUnknown: JET_ERRCAT = 0;
pub const JET_errcatError: JET_ERRCAT = 1;
pub const JET_errcatOperation: JET_ERRCAT = 2;
pub const JET_errcatFatal: JET_ERRCAT = 3;
pub const JET_errcatIO: JET_ERRCAT = 4;
pub const JET_errcatResource: JET_ERRCAT = 5;
pub const JET_errcatMemory: JET_ERRCAT = 6;
pub const JET_errcatQuota: JET_ERRCAT = 7;
pub const JET_errcatDisk: JET_ERRCAT = 8;
pub const JET_errcatData: JET_ERRCAT = 9;
pub const JET_errcatCorruption: JET_ERRCAT = 10;
pub const JET_errcatInconsistent: JET_ERRCAT = 11;
pub const JET_errcatFragmentation: JET_ERRCAT = 12;
pub const JET_errcatApi: JET_ERRCAT = 13;
pub const JET_errcatUsage: JET_ERRCAT = 14;
pub const JET_errcatState: JET_ERRCAT = 15;
pub const JET_errcatObsolete: JET_ERRCAT = 16;
pub const JET_errcatMax: JET_ERRCAT = 17;
pub const JET_instanceNil: JET_INSTANCE = !0;
pub const JET_sesidNil: JET_SESID = !0;
pub const JET_tableidNil: JET_TABLEID = !0;
pub const JET_bitNil: JET_GRBIT = 0;
pub const JET_LSNil: JET_LS = !0;
pub const JET_dbidNil: JET_DBID = 0xFFFFFFFF;
pub const JET_cbtypNull: JET_CBTYP = 0x00000000;
pub const JET_cbtypFinalize: JET_CBTYP = 0x00000001;
pub const JET_cbtypBeforeInsert: JET_CBTYP = 0x00000002;
pub const JET_cbtypAfterInsert: JET_CBTYP = 0x00000004;
pub const JET_cbtypBeforeReplace: JET_CBTYP = 0x00000008;
pub const JET_cbtypAfterReplace: JET_CBTYP = 0x00000010;
pub const JET_cbtypBeforeDelete: JET_CBTYP = 0x00000020;
pub const JET_cbtypAfterDelete: JET_CBTYP = 0x00000040;
pub const JET_cbtypUserDefinedDefaultValue: JET_CBTYP = 0x00000080;
pub const JET_cbtypOnlineDefragCompleted: JET_CBTYP = 0x00000100;
pub const JET_cbtypFreeCursorLS: JET_CBTYP = 0x00000200;
pub const JET_cbtypFreeTableLS: JET_CBTYP = 0x00000400;
pub const JET_coltypNil: JET_COLTYP = 0;
pub const JET_coltypBit: JET_COLTYP = 1;
pub const JET_coltypUnsignedByte: JET_COLTYP = 2;
pub const JET_coltypShort: JET_COLTYP = 3;
pub const JET_coltypLong: JET_COLTYP = 4;
pub const JET_coltypCurrency: JET_COLTYP = 5;
pub const JET_coltypIEEESingle: JET_COLTYP = 6;
pub const JET_coltypIEEEDouble: JET_COLTYP = 7;
pub const JET_coltypDateTime: JET_COLTYP = 8;
pub const JET_coltypBinary: JET_COLTYP = 9;
pub const JET_coltypText: JET_COLTYP = 10;
pub const JET_coltypLongBinary: JET_COLTYP = 11;
pub const JET_coltypLongText: JET_COLTYP = 12;
pub const JET_coltypSLV: JET_COLTYP = 13;
pub const JET_coltypUnsignedLong: JET_COLTYP = 14;
pub const JET_coltypLongLong: JET_COLTYP = 15;
pub const JET_coltypGUID: JET_COLTYP = 16;
pub const JET_coltypUnsignedShort: JET_COLTYP = 17;
pub const JET_coltypUnsignedLongLong: JET_COLTYP = 18;
pub const JET_coltypMax: JET_COLTYP = 19;
pub const JET_objtypNil: JET_OBJTYP = 0;
pub const JET_objtypTable: JET_OBJTYP = 1;
pub const JET_snpRepair: JET_SNP = 2;
pub const JET_snpCompact: JET_SNP = 4;
pub const JET_snpRestore: JET_SNP = 8;
pub const JET_snpBackup: JET_SNP = 9;
pub const JET_snpUpgrade: JET_SNP = 10;
pub const JET_snpScrub: JET_SNP = 11;
pub const JET_snpUpgradeRecordFormat: JET_SNP = 12;
pub const JET_sntBegin: JET_SNT = 5;
pub const JET_sntRequirements: JET_SNT = 7;
pub const JET_sntProgress: JET_SNT = 0;
pub const JET_sntComplete: JET_SNT = 6;
pub const JET_sntFail: JET_SNT = 3;
pub const JET_ExceptionMsgBox: JET_API_PTR = 1;
pub const JET_ExceptionNone: JET_API_PTR = 2;
pub const JET_EventLoggingDisable: JET_API_PTR = 0;
pub const JET_EventLoggingLevelMax: JET_API_PTR = 100;
pub type ParamId = ::ULONG;
pub const JET_paramSystemPath: ParamId = 0;
pub const JET_paramTempPath: ParamId = 1;
pub const JET_paramLogFilePath: ParamId = 2;
pub const JET_paramBaseName: ParamId = 3;
pub const JET_paramEventSource: ParamId = 4;
pub const JET_paramMaxSessions: ParamId = 5;
pub const JET_paramMaxOpenTables: ParamId = 6;
pub const JET_paramPreferredMaxOpenTables: ParamId = 7;
pub const JET_paramMaxCursors: ParamId = 8;
pub const JET_paramMaxVerPages: ParamId = 9;
pub const JET_paramMaxTemporaryTables: ParamId = 10;
pub const JET_paramLogFileSize: ParamId = 11;
pub const JET_paramLogBuffers: ParamId = 12;
pub const JET_paramWaitLogFlush: ParamId = 13;
pub const JET_paramLogCheckpointPeriod: ParamId = 14;
pub const JET_paramLogWaitingUserMax: ParamId = 15;
pub const JET_paramCommitDefault: ParamId = 16;
pub const JET_paramCircularLog: ParamId = 17;
pub const JET_paramDbExtensionSize: ParamId = 18;
pub const JET_paramPageTempDBMin: ParamId = 19;
pub const JET_paramPageFragment: ParamId = 20;
// 21: unused
pub const JET_paramBatchIOBufferMax: ParamId = 22;
pub const JET_paramCacheSizeMax: ParamId = 23;
pub const JET_paramCheckpointDepthMax: ParamId = 24;
pub const JET_paramLRUKCorrInterval: ParamId = 25;
pub const JET_paramLRUKHistoryMax: ParamId = 26;
pub const JET_paramLRUKPolicy: ParamId = 27;
pub const JET_paramLRUKTimeout: ParamId = 28;
pub const JET_paramLRUKTrxCorrInterval: ParamId = 29;
pub const JET_paramOutstandingIOMax: ParamId = 30;
pub const JET_paramStartFlushThreshold: ParamId = 31;
pub const JET_paramStopFlushThreshold: ParamId = 32;
// 33: unused
pub const JET_paramRecovery: ParamId = 34;
pub const JET_paramEnableOnlineDefrag: ParamId = 35;
// 36-40: unused
pub const JET_paramCacheSize: ParamId = 41;
// 42-43: unused
pub const JET_paramCheckFormatWhenOpenFail: ParamId = 44;
pub const JET_paramEnableIndexChecking: ParamId = 45;
pub const JET_paramEnableTempTableVersioning: ParamId = 46;
pub const JET_paramIgnoreLogVersion: ParamId = 47;
pub const JET_paramDeleteOldLogs: ParamId = 48;
pub const JET_paramEventSourceKey: ParamId = 49;
pub const JET_paramNoInformationEvent: ParamId = 50;
pub const JET_paramEventLoggingLevel: ParamId = 51;
pub const JET_paramDeleteOutOfRangeLogs: ParamId = 52;
pub const JET_paramAccessDeniedRetryPeriod: ParamId = 53;
pub const JET_paramEnableIndexCleanup: ParamId = 54;
// 55-59: unused
pub const JET_paramCacheSizeMin: ParamId = 60;
// 61-62: unused
pub const JET_paramPreferredVerPages: ParamId = 63;
pub const JET_paramDatabasePageSize: ParamId = 64;
pub const JET_paramDisableCallbacks: ParamId = 65;
// 66-68: unused
pub const JET_paramLogFileCreateAsynch: ParamId = 69;
pub const JET_paramErrorToString: ParamId = 70;
pub const JET_paramZeroDatabaseDuringBackup: ParamId = 71;
pub const JET_paramUnicodeIndexDefault: ParamId = 72;
pub const JET_paramRuntimeCallback: ParamId = 73;
// 74-76: unused
pub const JET_paramCleanupMismatchedLogFiles: ParamId = 77;
pub const JET_paramRecordUpgradeDirtyLevel: ParamId = 78;
// 79-80: unused
pub const JET_paramGlobalMinVerPages: ParamId = 81;
pub const JET_paramOSSnapshotTimeout: ParamId = 82;
// 83-97: unused
pub const JET_paramExceptionAction: ParamId = 98;
pub const JET_paramEventLogCache: ParamId = 99;
pub const JET_paramCreatePathIfNotExist: ParamId = 100;
pub const JET_paramPageHintCacheSize: ParamId = 101;
pub const JET_paramOneDatabasePerSession: ParamId = 102;
// 103: unused
pub const JET_paramMaxInstances: ParamId = 104;
pub const JET_paramVersionStoreTaskQueueMax: ParamId = 105;
// 106: unused
pub const JET_paramDisablePerfmon: ParamId = 107;
// 108-109: unused
pub const JET_paramIndexTuplesLengthMin: ParamId = 110;
pub const JET_paramIndexTuplesLengthMax: ParamId = 111;
pub const JET_paramIndexTuplesToIndexMax: ParamId = 112;
pub const JET_paramAlternateDatabaseRecoveryPath: ParamId = 113;
// 114-124: unused
pub const JET_paramCachedClosedTables: ParamId = 125;
pub const JET_paramEnableFileCache: ParamId = 126;
pub const JET_paramEnableViewCache: ParamId = 127;
pub const JET_paramVerPageSize: ParamId = 128;
pub const JET_paramConfiguration: ParamId = 129;
pub const JET_paramEnableAdvanced: ParamId = 130;
pub const JET_paramMaxColtyp: ParamId = 131;
pub const JET_paramIndexTupleIncrement: ParamId = 132;
pub const JET_paramIndexTupleStart: ParamId = 133;
pub const JET_paramKeyMost: ParamId = 134;
pub const JET_paramCheckpointIOMax: ParamId = 135;
pub const JET_paramLegacyFileNames: ParamId = 136;
// 137-151: deprecated JET_paramTableClass{n}Name
pub const JET_paramIOPriority: ParamId = 152;
pub const JET_paramWaypointLatency: ParamId = 153;
// 154-155: unused
pub const JET_paramEnablePersistedCallbacks: ParamId = 156;
// 157-159: unused
pub const JET_paramDefragmentSequentialBTrees: ParamId = 160;
pub const JET_paramDefragmentSequentialBTreesDensityCheckFrequency: ParamId = 161;
pub const JET_paramIOThrottlingTimeQuanta: ParamId = 162;
pub const JET_paramLVChunkSizeMost: ParamId = 163;
pub const JET_paramMaxCoalesceReadSize: ParamId = 164;
pub const JET_paramMaxCoalesceWriteSize: ParamId = 165;
pub const JET_paramMaxCoalesceReadGapSize: ParamId = 166;
pub const JET_paramMaxCoalesceWriteGapSize: ParamId = 167;
// the following aren't documented on MSDN, but are in the public header:
pub const JET_paramEnableDBScanInRecovery: ParamId = 169;
pub const JET_paramDbScanThrottle: ParamId = 170;
pub const JET_paramDbScanIntervalMinSec: ParamId = 171;
pub const JET_paramDbScanIntervalMaxSec: ParamId = 172;
// 173-176: unused
pub const JET_paramCachePriority: ParamId = 177;
pub const JET_paramMaxTransactionSize: ParamId = 178;
pub const JET_paramPrereadIOMax: ParamId = 179;
pub const JET_paramEnableDBScanSerialization: ParamId = 180;
pub const JET_paramHungIOThreshold: ParamId = 181;
pub const JET_paramHungIOActions: ParamId = 182;
pub const JET_paramMinDataForXpress: ParamId = 183;
pub const JET_paramEnableShinkDatabase: ParamId = 184;
// 185: reserved
pub const JET_paramProcessFriendlyName: ParamId = 186;
pub const JET_paramDurableCommitCallback: ParamId = 187;
pub const JET_paramEnableSqm: ParamId = 188;
pub const JET_paramConfigStoreSpec: ParamId = 189;

/* Flags for JET_paramLegacyFileNames */
pub const JET_bitESE98FileNames: JET_GRBIT = 			0x00000001;	//	Preserve the .log and .chk extension for compatibility reasons (i.e. Exchange)
pub const JET_bitEightDotThreeSoftCompat: JET_GRBIT = 	0x00000002;	//	Preserve the 8.3 naming syntax for as long as possible. (this should not be changed, w/o ensuring there are no log files)

/* Flags for JET_paramHungIOActions */
pub const JET_bitHungIOEvent: JET_GRBIT = 					0x00000001;	// Log event when an IO appears to be hung for over the IO threshold.

// Values for JET_paramEnableShrinkDatabase.
pub const JET_bitShrinkDatabaseOff: JET_GRBIT = 			0x0;
pub const JET_bitShrinkDatabaseOn: JET_GRBIT = 				0x1;		// Uses the file system's Sparse Files feature to release space in the middle of a file.
pub const JET_bitShrinkDatabaseRealtime: JET_GRBIT = 		0x2;		// Attempts to reclaim space back to the file system after freeing significant amounts of data (when space is marked as Available to the Root space tree).
// DEPRECATED:
pub const JET_bitShrinkDatabaseTrim: JET_GRBIT = 			0x1;		// Deprecated value for JET_bitShrinkDatabaseOn; Will be removed!

/* Flags for JetInit2, JetInit3 */
pub const JET_bitReplayIgnoreMissingDB: JET_GRBIT = 		0x00000004;	//	Ignore missing databases during recovery. This is a very dangerous option and may irrevocably produce an inconsistent database if improperly used. Normal ESE usage does not typically require this dangerous option.
pub const JET_bitRecoveryWithoutUndo: JET_GRBIT = 			0x00000008;	//	perform recovery, but halt at the Undo phase
pub const JET_bitTruncateLogsAfterRecovery: JET_GRBIT = 	0x00000010;	//	on successful soft recovery, truncate log files
pub const JET_bitReplayMissingMapEntryDB: JET_GRBIT = 		0x00000020;	//	missing database map entry default to same location
pub const JET_bitLogStreamMustExist: JET_GRBIT = 			0x00000040;	//	transaction logs must exist in the logfile directory (ie. cannot auto-start a new log stream)
pub const JET_bitReplayIgnoreLostLogs: JET_GRBIT = 			0x00000080;	//	ignore logs lost from the end of the log stream
pub const JET_bitKeepDbAttachedAtEndOfRecovery: JET_GRBIT =  0x00001000; //	this allows db to remain attached at the end of recovery (for faster transition to running state)

/* Flags for JetTerm2 */
pub const JET_bitTermComplete: JET_GRBIT = 				0x00000001;
pub const JET_bitTermAbrupt: JET_GRBIT = 				0x00000002;
pub const JET_bitTermStopBackup: JET_GRBIT = 			0x00000004;
pub const JET_bitTermDirty: JET_GRBIT = 				0x00000008;

/* Flags for JetIdle */
pub const JET_bitIdleFlushBuffers: JET_GRBIT = 			0x00000001;
pub const JET_bitIdleCompact: JET_GRBIT = 				0x00000002;
pub const JET_bitIdleStatus: JET_GRBIT = 				0x00000004;

/* Flags for JetEndSession */

/* Flags for JetAttachDatabase/JetOpenDatabase */
pub const JET_bitDbReadOnly: JET_GRBIT = 				0x00000001;
pub const JET_bitDbExclusive: JET_GRBIT = 				0x00000002; /* multiple opens allowed */
pub const JET_bitDbDeleteCorruptIndexes: JET_GRBIT = 	0x00000010; /* delete indexes possibly corrupted by NT version upgrade */
pub const JET_bitDbDeleteUnicodeIndexes: JET_GRBIT = 	0x00000400; /* delete all indexes with unicode columns */
pub const JET_bitDbUpgrade: JET_GRBIT = 				0x00000200; /* */
pub const JET_bitDbEnableBackgroundMaintenance: JET_GRBIT = 	0x00000800;	/* the database engine will initiate automatic background database maintenance */
pub const JET_bitDbPurgeCacheOnAttach: JET_GRBIT = 		0x00001000; /* used to ensure any kept alive cache is purged for this DB before attach */

/* Flags for JetDetachDatabase2 */
pub const JET_bitForceDetach: JET_GRBIT = 			  		0x00000001;
pub const JET_bitForceCloseAndDetach: JET_GRBIT = 			(0x00000002 | JET_bitForceDetach);

/* Flags for JetCreateDatabase */
pub const JET_bitDbRecoveryOff: JET_GRBIT =  			0x00000008; /* disable logging/recovery for this database */
pub const JET_bitDbShadowingOff: JET_GRBIT = 			0x00000080; /* disable catalog shadowing */
pub const JET_bitDbOverwriteExisting: JET_GRBIT = 		0x00000200; /* overwrite existing database with same name */

/* Flags for JetBackup, JetBeginExternalBackup, JetBeginExternalBackupInstance, JetBeginSurrogateBackup */
pub const JET_bitBackupIncremental: JET_GRBIT = 		0x00000001;
pub const JET_bitBackupAtomic: JET_GRBIT = 				0x00000004;
pub const JET_bitBackupSnapshot: JET_GRBIT = 			0x00000010;

/* Flags for JetEndExternalBackupInstance2, JetEndSurrogateBackup */
pub const JET_bitBackupEndNormal: JET_GRBIT = 				0x0001;
pub const JET_bitBackupEndAbort: JET_GRBIT = 				0x0002;
pub const JET_bitBackupTruncateDone: JET_GRBIT = 			0x0100;

/* Flags for JetCreateTableColumnIndex */
pub const JET_bitTableCreateFixedDDL: JET_GRBIT = 			0x00000001;	/* DDL is fixed */
pub const JET_bitTableCreateTemplateTable: JET_GRBIT = 		0x00000002;	/* DDL is inheritable (implies FixedDDL) */
pub const JET_bitTableCreateNoFixedVarColumnsInDerivedTables: JET_GRBIT = 	0x00000004;
                                                    //	used in conjunction with JET_bitTableCreateTemplateTable
                                                    //	to disallow fixed/var columns in derived tables (so that
                                                    //	fixed/var columns may be added to the template in the future)
pub const JET_bitTableCreateImmutableStructure: JET_GRBIT = 	0x00000008;	// Do not write to the input structures. Additionally, do not return any auto-opened tableid.

/* Flags for JetAddColumn, JetGetColumnInfo, JetOpenTempTable */
pub const JET_bitColumnFixed: JET_GRBIT = 				0x00000001;
pub const JET_bitColumnTagged: JET_GRBIT = 				0x00000002;
pub const JET_bitColumnNotNULL: JET_GRBIT = 			0x00000004;
pub const JET_bitColumnVersion: JET_GRBIT = 				0x00000008;
pub const JET_bitColumnAutoincrement: JET_GRBIT = 		0x00000010;
pub const JET_bitColumnUpdatable: JET_GRBIT = 			0x00000020; /* JetGetColumnInfo only */
pub const JET_bitColumnTTKey: JET_GRBIT = 				0x00000040; /* JetOpenTempTable only */
pub const JET_bitColumnTTDescending: JET_GRBIT = 		0x00000080; /* JetOpenTempTable only */
pub const JET_bitColumnMultiValued: JET_GRBIT = 			0x00000400;
pub const JET_bitColumnEscrowUpdate: JET_GRBIT = 		0x00000800; /* escrow updated */
pub const JET_bitColumnUnversioned: JET_GRBIT = 		0x00001000; /* for add column only - add column unversioned */
pub const JET_bitColumnMaybeNull: JET_GRBIT = 			0x00002000; /* for retrieve column info of outer join where no match from the inner table */
pub const JET_bitColumnFinalize: JET_GRBIT = 				0x00004000; /* this is a finalizable column (issue callback if escrow value equals 0) */
pub const JET_bitColumnUserDefinedDefault: JET_GRBIT = 	0x00008000; /* default value from a user-provided callback */
pub const JET_bitColumnDeleteOnZero: JET_GRBIT = 		0x00020000; /* this is a finalizable column (delete record if escrow value equals 0) */
pub const JET_bitColumnCompressed: JET_GRBIT = 			0x00080000; /* data in the column can be compressed */

//	flags for JetDeleteColumn
pub const JET_bitDeleteColumnIgnoreTemplateColumns: JET_GRBIT = 	0x00000001;	//	for derived tables, don't bother looking in template columns

/* Flags for JetSetCurrentIndex */
pub const JET_bitMoveFirst: JET_GRBIT = 				0x00000000;
pub const JET_bitNoMove: JET_GRBIT = 					0x00000002;

/* Flags for JetMakeKey */
pub const JET_bitNewKey: JET_GRBIT = 					0x00000001;
pub const JET_bitStrLimit: JET_GRBIT =  				0x00000002;
pub const JET_bitSubStrLimit: JET_GRBIT = 				0x00000004;
pub const JET_bitNormalizedKey: JET_GRBIT =  			0x00000008;
pub const JET_bitKeyDataZeroLength: JET_GRBIT = 		0x00000010;
pub const JET_bitFullColumnStartLimit: JET_GRBIT = 		0x00000100;
pub const JET_bitFullColumnEndLimit: JET_GRBIT = 		0x00000200;
pub const JET_bitPartialColumnStartLimit: JET_GRBIT = 	0x00000400;
pub const JET_bitPartialColumnEndLimit: JET_GRBIT = 	0x00000800;

/* Flags for JetSetIndexRange */
pub const JET_bitRangeInclusive: JET_GRBIT = 			0x00000001;
pub const JET_bitRangeUpperLimit: JET_GRBIT = 			0x00000002;
pub const JET_bitRangeInstantDuration: JET_GRBIT = 		0x00000004;
pub const JET_bitRangeRemove: JET_GRBIT = 				0x00000008;

/* Flags for JetGetLock */
pub const JET_bitReadLock: JET_GRBIT = 					0x00000001;
pub const JET_bitWriteLock: JET_GRBIT = 				0x00000002;

/* Constants for JetMove */
pub const JET_MoveFirst: JET_GRBIT = 					(0x80000000);
pub const JET_MovePrevious: JET_GRBIT = 				(!1);
pub const JET_MoveNext: JET_GRBIT = 					(1);
pub const JET_MoveLast: JET_GRBIT = 					(0x7fffffff);

/* Flags for JetMove */
pub const JET_bitMoveKeyNE: JET_GRBIT = 				0x00000001;

/* Flags for JetSeek */
pub const JET_bitSeekEQ: JET_GRBIT = 					0x00000001;
pub const JET_bitSeekLT: JET_GRBIT = 					0x00000002;
pub const JET_bitSeekLE: JET_GRBIT = 					0x00000004;
pub const JET_bitSeekGE: JET_GRBIT = 					0x00000008;
pub const JET_bitSeekGT: JET_GRBIT = 		 			0x00000010;
pub const JET_bitSetIndexRange: JET_GRBIT = 			0x00000020;
pub const JET_bitCheckUniqueness: JET_GRBIT = 			0x00000040;	//	to be used with JET_bitSeekEQ only, returns JET_wrnUniqueKey if seek lands on a key which has no dupes

//	Flags for JetGotoSecondaryIndexBookmark
pub const JET_bitBookmarkPermitVirtualCurrency: JET_GRBIT = 	0x00000001;	//	place cursor on relative position in index if specified bookmark no longer exists

/* Flags for JET_CONDITIONALCOLUMN */
pub const JET_bitIndexColumnMustBeNull: JET_GRBIT = 	0x00000001;
pub const JET_bitIndexColumnMustBeNonNull: JET_GRBIT = 	0x00000002;

/* Flags for JET_INDEXRANGE */
pub const JET_bitRecordInIndex: JET_GRBIT = 			0x00000001;
pub const JET_bitRecordNotInIndex: JET_GRBIT = 			0x00000002;

/* Flags for JetCreateIndex */
pub const JET_bitIndexUnique: JET_GRBIT = 				0x00000001;
pub const JET_bitIndexPrimary: JET_GRBIT = 				0x00000002;
pub const JET_bitIndexDisallowNull: JET_GRBIT = 		0x00000004;
pub const JET_bitIndexIgnoreNull: JET_GRBIT = 			0x00000008;
pub const JET_bitIndexIgnoreAnyNull: JET_GRBIT = 		0x00000020;
pub const JET_bitIndexIgnoreFirstNull: JET_GRBIT = 		0x00000040;
pub const JET_bitIndexLazyFlush: JET_GRBIT = 			0x00000080;
pub const JET_bitIndexEmpty: JET_GRBIT = 				0x00000100;	// don't attempt to build index, because all entries would evaluate to NULL (MUST also specify JET_bitIgnoreAnyNull)
pub const JET_bitIndexUnversioned: JET_GRBIT = 			0x00000200;
pub const JET_bitIndexSortNullsHigh: JET_GRBIT = 		0x00000400;	// NULL sorts after data for all columns in the index
pub const JET_bitIndexUnicode: JET_GRBIT = 				0x00000800;	// LCID field of JET_INDEXCREATE actually points to a JET_UNICODEINDEX struct to allow user-defined LCMapString() flags
pub const JET_bitIndexTuples: JET_GRBIT = 				0x00001000;	// index on substring tuples (text columns only)
pub const JET_bitIndexTupleLimits: JET_GRBIT = 			0x00002000;	// cbVarSegMac field of JET_INDEXCREATE actually points to a JET_TUPLELIMITS struct to allow custom tuple index limits (implies JET_bitIndexTuples)
pub const JET_bitIndexCrossProduct: JET_GRBIT = 		0x00004000;	// index over multiple multi-valued columns has full cross product
pub const JET_bitIndexKeyMost: JET_GRBIT = 				0x00008000;	// custom index key size set instead of default of 255 bytes
pub const JET_bitIndexDisallowTruncation: JET_GRBIT = 	0x00010000;	// fail update rather than truncate index keys
pub const JET_bitIndexNestedTable: JET_GRBIT = 			0x00020000;	// index over multiple multi-valued columns but only with values of same itagSequence
pub const JET_bitIndexDotNetGuid: JET_GRBIT = 			0x00040000;  // index over GUID column according to .Net GUID sort order
pub const JET_bitIndexImmutableStructure: JET_GRBIT = 	0x00080000;	// Do not write to the input structures during a JetCreateIndexN call.

/* Flags for index key definition */
pub const JET_bitKeyAscending: JET_GRBIT = 				0x00000000;
pub const JET_bitKeyDescending: JET_GRBIT = 			0x00000001;

/* Flags for JetOpenTable */
pub const JET_bitTableDenyWrite: JET_GRBIT = 			0x00000001;
pub const JET_bitTableDenyRead: JET_GRBIT = 			0x00000002;
pub const JET_bitTableReadOnly: JET_GRBIT = 			0x00000004;
pub const JET_bitTableUpdatable: JET_GRBIT = 			0x00000008;
pub const JET_bitTablePermitDDL: JET_GRBIT = 			0x00000010;	/*  override table flagged as FixedDDL (must be used with DenyRead) */
pub const JET_bitTableNoCache: JET_GRBIT = 				0x00000020;	/*	don't cache the pages for this table */
pub const JET_bitTablePreread: JET_GRBIT = 				0x00000040;	/*	assume the table is probably not in the buffer cache */
pub const JET_bitTableOpportuneRead: JET_GRBIT = 		0x00000080;	/*	attempt to opportunely read physically adjacent leaf pages using larger physical IOs */
pub const JET_bitTableSequential: JET_GRBIT = 			0x00008000;  /*  assume the table will be scanned sequentially */

pub const JET_bitTableClassMask: JET_GRBIT = 		0x000F0000;	/*  table stats class mask  */
pub const JET_bitTableClassNone: JET_GRBIT = 		0x00000000;  /*  table belongs to no stats class (default)  */
pub const JET_bitTableClass1: JET_GRBIT = 			0x00010000;  /*  table belongs to stats class 1  */
pub const JET_bitTableClass2: JET_GRBIT = 			0x00020000;  /*  table belongs to stats class 2  */
pub const JET_bitTableClass3: JET_GRBIT = 			0x00030000;  /*  table belongs to stats class 3  */
pub const JET_bitTableClass4: JET_GRBIT = 			0x00040000;  /*  table belongs to stats class 4  */
pub const JET_bitTableClass5: JET_GRBIT = 			0x00050000;  /*  table belongs to stats class 5  */
pub const JET_bitTableClass6: JET_GRBIT = 			0x00060000;  /*  table belongs to stats class 6  */
pub const JET_bitTableClass7: JET_GRBIT = 			0x00070000;  /*  table belongs to stats class 7  */
pub const JET_bitTableClass8: JET_GRBIT = 			0x00080000;  /*  table belongs to stats class 8  */
pub const JET_bitTableClass9: JET_GRBIT = 			0x00090000;  /*  table belongs to stats class 9  */
pub const JET_bitTableClass10: JET_GRBIT = 			0x000A0000;  /*  table belongs to stats class 10  */
pub const JET_bitTableClass11: JET_GRBIT = 			0x000B0000;  /*  table belongs to stats class 11  */
pub const JET_bitTableClass12: JET_GRBIT = 			0x000C0000;  /*  table belongs to stats class 12  */
pub const JET_bitTableClass13: JET_GRBIT = 			0x000D0000;  /*  table belongs to stats class 13  */
pub const JET_bitTableClass14: JET_GRBIT = 			0x000E0000;  /*  table belongs to stats class 14  */
pub const JET_bitTableClass15: JET_GRBIT = 			0x000F0000;  /*  table belongs to stats class 15  */

pub const JET_bitLSReset: JET_GRBIT = 				0x00000001;	/*	reset LS value */
pub const JET_bitLSCursor: JET_GRBIT = 				0x00000002;	/*	set/retrieve LS of table cursor */
pub const JET_bitLSTable: JET_GRBIT = 				0x00000004;	/*	set/retrieve LS of table */

/* Flags for JetSetTableSequential and JetPrereadIndexRanges */
pub const JET_bitPrereadForward: JET_GRBIT = 		0x00000001;	/*	Hint that the sequential traversal will be in the forward direction */
pub const JET_bitPrereadBackward: JET_GRBIT = 		0x00000002;	/*	Hint that the sequential traversal will be in the backward direction */
pub const JET_bitPrereadFirstPage: JET_GRBIT = 		0x00000004;	/*	Only first page of long values should be preread */
pub const JET_bitPrereadNormalizedKey: JET_GRBIT = 	0x00000008;	/*	Normalized key/bookmark provided instead of column value */

/* Flags for JetOpenTempTable */
pub const JET_bitTTIndexed: JET_GRBIT = 			0x00000001;	/* Allow seek */
pub const JET_bitTTUnique: JET_GRBIT =  			0x00000002;	/* Remove duplicates */
pub const JET_bitTTUpdatable: JET_GRBIT = 			0x00000004;	/* Allow updates */
pub const JET_bitTTScrollable: JET_GRBIT = 			0x00000008;	/* Allow backwards scrolling */
pub const JET_bitTTSortNullsHigh: JET_GRBIT = 		0x00000010;	/* NULL sorts after data for all columns in the index */
pub const JET_bitTTForceMaterialization: JET_GRBIT = 		0x00000020;						/* Forces temp. table to be materialized into a btree (allows for duplicate detection) */
pub const JET_bitTTErrorOnDuplicateInsertion: JET_GRBIT = 	JET_bitTTForceMaterialization;	/* Error always returned when duplicate is inserted (instead of dupe being silently removed) */
pub const JET_bitTTForwardOnly: JET_GRBIT = 		0x00000040;	/* Prevents temp. table from being materialized into a btree (and enables duplicate keys) */
pub const JET_bitTTIntrinsicLVsOnly: JET_GRBIT = 	0x00000080;	//	permit only intrinsic LV's (so materialisation is not required simply because a TT has an LV column)
pub const JET_bitTTDotNetGuid: JET_GRBIT = 			0x00000100;	//	sort all JET_coltypGUID columns according to .Net Guid sort order

/* Flags for JetSetColumn */
pub const JET_bitSetAppendLV: JET_GRBIT = 					0x00000001;
pub const JET_bitSetOverwriteLV: JET_GRBIT = 				0x00000004; /* overwrite JET_coltypLong* byte range */
pub const JET_bitSetSizeLV: JET_GRBIT = 					0x00000008; /* set JET_coltypLong* size */
pub const JET_bitSetZeroLength: JET_GRBIT = 				0x00000020;
pub const JET_bitSetSeparateLV: JET_GRBIT =  				0x00000040; /* force LV separation */
pub const JET_bitSetUniqueMultiValues: JET_GRBIT = 			0x00000080; /* prevent duplicate multi-values */
pub const JET_bitSetUniqueNormalizedMultiValues: JET_GRBIT = 	0x00000100; /* prevent duplicate multi-values, normalizing all data before performing comparisons */
pub const JET_bitSetRevertToDefaultValue: JET_GRBIT = 		0x00000200; /* if setting last tagged instance to NULL, revert to default value instead if one exists */
pub const JET_bitSetIntrinsicLV: JET_GRBIT = 				0x00000400; /* store whole LV in record without bursting or return an error */
pub const JET_bitSetCompressed: JET_GRBIT = 				0x00020000; /* attempt compression when storing the data */
pub const JET_bitSetUncompressed: JET_GRBIT = 				0x00010000; /* don't attempt compression when storing the data */

/*	Space Hint Flags / JET_SPACEHINTS	*/
//	Generic
pub const JET_bitSpaceHintsUtilizeParentSpace: JET_GRBIT = 			0x00000001;	//	This changes the internal allocation policy to get space heirarchically from a B-Tree's immediate parent.
//	Create
pub const JET_bitCreateHintAppendSequential: JET_GRBIT = 			0x00000002;	//	This bit will enable Append split behavior to grow according to the growth dynamics of the table (set by cbMinExtent, ulGrowth, cbMaxExtent).
pub const JET_bitCreateHintHotpointSequential: JET_GRBIT = 			0x00000004;	//	This bit will enable Hotpoint split behavior to grow according to the growth dynamics of the table (set by cbMinExtent, ulGrowth, cbMaxExtent).
//	Retrieve
pub const JET_bitRetrieveHintReserve1: JET_GRBIT = 					0x00000008;	//	Reserved and ignored
pub const JET_bitRetrieveHintTableScanForward: JET_GRBIT = 			0x00000010;	//	By setting this the client indicates that forward sequential scan is the predominant usage pattern of this table.
pub const JET_bitRetrieveHintTableScanBackward: JET_GRBIT = 		0x00000020;	//	By setting this the client indicates that backwards sequential scan is the predominant usage pattern of this table.
pub const JET_bitRetrieveHintReserve2: JET_GRBIT = 					0x00000040;	//	Reserved and ignored
pub const JET_bitRetrieveHintReserve3: JET_GRBIT = 					0x00000080;	//	Reserved and ignored
//	Update
//pub const JET_bitUpdateReserved: JET_GRBIT = 						0x00000000;	//	TBD.
//	Delete / .grbitDelete
pub const JET_bitDeleteHintTableSequential: JET_GRBIT = 			0x00000100;	//	This means that the application expects this table to be cleaned up in-order sequentially (from lowest key to highest key)

/* Options for JetPrepareUpdate */
/*
#define JET_prepInsert						0
#define JET_prepReplace 					2
#define JET_prepCancel						3
#define JET_prepReplaceNoLock				4
#define JET_prepInsertCopy					5
#define JET_prepInsertCopyDeleteOriginal	7	//	used for updating a record in the primary key; avoids the delete/insert process and updates autoinc
#define JET_prepInsertCopyReplaceOriginal	9	//	used for updating a record in the primary key; avoids the delete/insert process and keeps autoinc
*/

	//	Flags for JetUpdate
pub const JET_bitUpdateCheckESE97Compatibility: JET_GRBIT = 	0x00000001;	//	check whether record fits if represented in ESE97 database format

	/* Flags for JetEscrowUpdate */
pub const JET_bitEscrowNoRollback: JET_GRBIT = 				0x0001;

	/* Flags for JetRetrieveColumn */
pub const JET_bitRetrieveCopy: JET_GRBIT = 					0x00000001;
pub const JET_bitRetrieveFromIndex: JET_GRBIT = 			0x00000002;
pub const JET_bitRetrieveFromPrimaryBookmark: JET_GRBIT = 	0x00000004;
pub const JET_bitRetrieveTag: JET_GRBIT = 					0x00000008;
pub const JET_bitRetrieveNull: JET_GRBIT = 					0x00000010;	/*	for columnid 0 only */
pub const JET_bitRetrieveIgnoreDefault: JET_GRBIT = 		0x00000020;	/*	for columnid 0 only */
pub const JET_bitRetrieveLongId: JET_GRBIT = 				0x00000040;
pub const JET_bitRetrieveLongValueRefCount: JET_GRBIT = 	0x00000080;	/*  for testing use only */
pub const JET_bitRetrieveTuple: JET_GRBIT = 				0x00000800; /* retrieve tuple fragment from index */

	/* Flags for JET_INDEX_COLUMN */
pub const JET_bitZeroLength: JET_GRBIT = 					0x00000001;

/* Flags for JetEnumerateColumns */
pub const JET_bitEnumerateCopy: JET_GRBIT = 						JET_bitRetrieveCopy;
pub const JET_bitEnumerateIgnoreDefault: JET_GRBIT = 				JET_bitRetrieveIgnoreDefault;
pub const JET_bitEnumeratePresenceOnly: JET_GRBIT = 				0x00020000;
pub const JET_bitEnumerateTaggedOnly: JET_GRBIT = 					0x00040000;
pub const JET_bitEnumerateCompressOutput: JET_GRBIT = 				0x00080000;
// Available on Server 2003 SP1
pub const JET_bitEnumerateIgnoreUserDefinedDefault: JET_GRBIT = 	0x00100000;
pub const JET_bitEnumerateInRecordOnly: JET_GRBIT = 				0x00200000;

/* Flags for JetGetRecordSize */
pub const JET_bitRecordSizeInCopyBuffer: JET_GRBIT = 			0x00000001;	//	use record in copy buffer
pub const JET_bitRecordSizeRunningTotal: JET_GRBIT = 			0x00000002;	//	increment totals in output buffer instead of setting them
pub const JET_bitRecordSizeLocal: JET_GRBIT = 					0x00000004;	//	ignore Long Values (and other data otherwise not in the same page as the record)

/* Flags for JetBeginTransaction2 */
pub const JET_bitTransactionReadOnly: JET_GRBIT = 		0x00000001;	/* transaction will not modify the database */

	/* Flags for JetCommitTransaction */
pub const JET_bitCommitLazyFlush: JET_GRBIT = 			0x00000001;	/* lazy flush log buffers. */
pub const JET_bitWaitLastLevel0Commit: JET_GRBIT = 		0x00000002;	/* wait for last level 0 commit record flushed */
pub const JET_bitWaitAllLevel0Commit: JET_GRBIT = 		0x00000008;	/* wait for all level 0 commits to be flushed */
pub const JET_bitForceNewLog: JET_GRBIT = 				0x00000010;

	/* Flags for JetRollback */
pub const JET_bitRollbackAll: JET_GRBIT = 				0x00000001;

	/* Flags for JetOSSnapshot APIs */

	/* Flags for JetOSSnapshotPrepare */
pub const JET_bitIncrementalSnapshot: JET_GRBIT = 		0x00000001;	/* bit 0: full (0) or incremental (1) snapshot */
pub const JET_bitCopySnapshot: JET_GRBIT = 				0x00000002;	/* bit 1: normal (0) or copy (1) snapshot */
pub const JET_bitContinueAfterThaw: JET_GRBIT = 		0x00000004;	/* bit 2: end on thaw (0) or wait for [truncate +] end snapshot */
pub const JET_bitExplicitPrepare: JET_GRBIT = 			0x00000008;	/* bit 3: all instaces prepared by default (0) or no instance prepared by default (1)  */

	/* Flags for JetOSSnapshotTruncateLog & JetOSSnapshotTruncateLogInstance */
pub const JET_bitAllDatabasesSnapshot: JET_GRBIT = 		0x00000001;	/* bit 0: there are detached dbs in the instance (i.e. can't truncate logs) */

	/* Flags for JetOSSnapshotEnd */
pub const JET_bitAbortSnapshot: JET_GRBIT = 			0x00000001;  /* snapshot process failed */

// grbits for JetResizeDatabase:
pub const JET_bitResizeDatabaseOnlyGrow: JET_GRBIT = 				0x00000001;	// Only grow the database. If the resize call would shrink the database, do nothing.
pub const JET_bitResizeDatabaseOnlyShrink: JET_GRBIT = 				0x00000002;	// Only shrink the database, but keeping an empty extent at the end. If the resize call would grow the database, do nothing. The file may end up smaller than requested.
pub const JET_bitStopServiceAll: JET_GRBIT = 						0x00000000;	//	Stops all ESE services for the specified instance.
pub const JET_bitStopServiceBackgroundUserTasks: JET_GRBIT = 		0x00000002;	//	Stops restartable client specificed background maintenance tasks (B+ Tree Defrag for example).
pub const JET_bitStopServiceQuiesceCaches: JET_GRBIT = 				0x00000004;	//	Quiesces all dirty caches to disk. Asynchronous. Cancellable.
// Warning: This bit can only be used to resume StopServiceBackgroundUserTasks and JET_bitStopServiceQuiesceCaches, if you
// previously called with JET_bitStopServiceAll, attempting to use JET_bitStopServiceResume will fail.
pub const JET_bitStopServiceResume: JET_GRBIT = 					0x80000000;	//	Resumes previously issued StopService operations, i.e. "restarts service".  Can be combined with above grbits to Resume specific services, or with JET_bitStopServiceAll to Resume all previously stopped services.

/**********************************************************************/
/***********************     ERROR CODES     **************************/
/**********************************************************************/

// SUCCESS

pub const JET_errSuccess: JET_ERR=						 0;    /* Successful Operation */

// ERRORS

pub const JET_wrnNyi: JET_ERR=							-1;    /* Function Not Yet Implemented */

//	SYSTEM errors
pub const JET_errRfsFailure: JET_ERR=					-100;  /* Resource Failure Simulator failure */
pub const JET_errRfsNotArmed: JET_ERR=					-101;  /* Resource Failure Simulator not initialized */
pub const JET_errFileClose: JET_ERR=					-102;  /* Could not close file */
pub const JET_errOutOfThreads: JET_ERR=					-103;  /* Could not start thread */
pub const JET_errTooManyIO: JET_ERR=					-105;  /* System busy due to too many IOs */
pub const JET_errTaskDropped: JET_ERR=					-106;  /* A requested async task could not be executed */
pub const JET_errInternalError: JET_ERR=				-107;  /* Fatal internal error */
pub const JET_errDisabledFunctionality: JET_ERR=		-112;  /* You are running MinESE, that does not have all features compiled in.  This functionality is only supported in a full version of ESE. */
pub const JET_errUnloadableOSFunctionality: JET_ERR=	-113;  /* The desired OS functionality could not be located and loaded / linked. */

//	BUFFER MANAGER errors
pub const JET_errDatabaseBufferDependenciesCorrupted: JET_ERR=	-255;	/* Buffer dependencies improperly set. Recovery failure */

//	DIRECTORY MANAGER errors
pub const JET_wrnRemainingVersions: JET_ERR= 			 321;  /* The version store is still active */
pub const JET_errPreviousVersion: JET_ERR=				-322;  /* Version already existed. Recovery failure */
pub const JET_errPageBoundary: JET_ERR=					-323;  /* Reached Page Boundary */
pub const JET_errKeyBoundary: JET_ERR=		  			-324;  /* Reached Key Boundary */
pub const JET_errBadPageLink: JET_ERR=					-327;  /* Database corrupted */
pub const JET_errBadBookmark: JET_ERR=					-328;  /* Bookmark has no corresponding address in database */
pub const JET_errNTSystemCallFailed: JET_ERR= 			-334;  // A call to the operating system failed
pub const JET_errBadParentPageLink: JET_ERR=			-338;  // Database corrupted
pub const JET_errSPAvailExtCacheOutOfSync: JET_ERR=		-340;  // AvailExt cache doesn't match btree
pub const JET_errSPAvailExtCorrupted: JET_ERR=			-341;  // AvailExt space tree is corrupt
pub const JET_errSPAvailExtCacheOutOfMemory: JET_ERR=	-342;  // Out of memory allocating an AvailExt cache node
pub const JET_errSPOwnExtCorrupted: JET_ERR=			-343;  // OwnExt space tree is corrupt
pub const JET_errDbTimeCorrupted: JET_ERR=				-344;  // Dbtime on current page is greater than global database dbtime
pub const JET_wrnUniqueKey: JET_ERR=					 345;  // seek on non-unique index yielded a unique key
pub const JET_errKeyTruncated: JET_ERR=					-346;  // key truncated on index that disallows key truncation
pub const JET_errDatabaseLeakInSpace: JET_ERR=			-348;  // Some database pages have become unreachable even from the avail tree, only an offline defragmentation can return the lost space.
pub const JET_errBadEmptyPage: JET_ERR=					-351;  // Database corrupted. Searching an unexpectedly empty page.

//	RECORD MANAGER errors
pub const JET_wrnSeparateLongValue: JET_ERR=			 406;  /* Column is a separated long-value */
pub const JET_wrnRecordFoundGreater: JET_ERR=			JET_wrnSeekNotEqual;
pub const JET_wrnRecordFoundLess: JET_ERR=    			JET_wrnSeekNotEqual;
pub const JET_errColumnIllegalNull: JET_ERR=  			JET_errNullInvalid;
pub const JET_errKeyTooBig: JET_ERR=					-408;  /* Key is too large */
pub const JET_errCannotSeparateIntrinsicLV: JET_ERR=	-416;	// illegal attempt to separate an LV which must be intrinsic
pub const JET_errSeparatedLongValue: JET_ERR=			-421; /* Operation not supported on separated long-value */
pub const JET_errMustBeSeparateLongValue: JET_ERR=		-423;  /* Can only preread long value columns that can be separate, e.g. not size constrained so that they are fixed or variable columns */
pub const JET_errInvalidPreread: JET_ERR=				-424;  /* Cannot preread long values when current index secondary */

//	LOGGING/RECOVERY errors
pub const JET_errInvalidLoggedOperation: JET_ERR=		-500;  /* Logged operation cannot be redone */
pub const JET_errLogFileCorrupt: JET_ERR=		  		-501;  /* Log file is corrupt */
pub const JET_errNoBackupDirectory: JET_ERR= 			-503;  /* No backup directory given */
pub const JET_errBackupDirectoryNotEmpty: JET_ERR= 		-504;  /* The backup directory is not emtpy */
pub const JET_errBackupInProgress: JET_ERR= 			-505;  /* Backup is active already */
pub const JET_errRestoreInProgress: JET_ERR=			-506;  /* Restore in progress */
pub const JET_errMissingPreviousLogFile: JET_ERR=		-509;  /* Missing the log file for check point */
pub const JET_errLogWriteFail: JET_ERR=					-510;  /* Failure writing to log file */
pub const JET_errLogDisabledDueToRecoveryFailure: JET_ERR=	-511; /* Try to log something after recovery faild */
pub const JET_errCannotLogDuringRecoveryRedo: JET_ERR=		-512;	/* Try to log something during recovery redo */
pub const JET_errLogGenerationMismatch: JET_ERR=		-513;  /* Name of logfile does not match internal generation number */
pub const JET_errBadLogVersion: JET_ERR=  	  			-514;  /* Version of log file is not compatible with Jet version */
pub const JET_errInvalidLogSequence: JET_ERR=  			-515;  /* Timestamp in next log does not match expected */
pub const JET_errLoggingDisabled: JET_ERR= 				-516;  /* Log is not active */
pub const JET_errLogBufferTooSmall: JET_ERR=			-517;  /* Log buffer is too small for recovery */
pub const JET_errLogSequenceEnd: JET_ERR=				-519;  /* Maximum log file number exceeded */
pub const JET_errNoBackup: JET_ERR=						-520;  /* No backup in progress */
pub const JET_errInvalidBackupSequence: JET_ERR=		-521;  /* Backup call out of sequence */
pub const JET_errBackupNotAllowedYet: JET_ERR=			-523;  /* Cannot do backup now */
pub const JET_errDeleteBackupFileFail: JET_ERR=	   		-524;  /* Could not delete backup file */
pub const JET_errMakeBackupDirectoryFail: JET_ERR= 		-525;  /* Could not make backup temp directory */
pub const JET_errInvalidBackup: JET_ERR=		 		-526;  /* Cannot perform incremental backup when circular logging enabled */
pub const JET_errRecoveredWithErrors: JET_ERR=			-527;  /* Restored with errors */
pub const JET_errMissingLogFile: JET_ERR=				-528;  /* Current log file missing */
pub const JET_errLogDiskFull: JET_ERR=					-529;  /* Log disk full */
pub const JET_errBadLogSignature: JET_ERR=				-530;  /* Bad signature for a log file */
pub const JET_errBadDbSignature: JET_ERR=				-531;  /* Bad signature for a db file */
pub const JET_errBadCheckpointSignature: JET_ERR=		-532;  /* Bad signature for a checkpoint file */
pub const JET_errCheckpointCorrupt: JET_ERR=			-533;  /* Checkpoint file not found or corrupt */
pub const JET_errMissingPatchPage: JET_ERR=				-534;  /* Patch file page not found during recovery */
pub const JET_errBadPatchPage: JET_ERR=					-535;  /* Patch file page is not valid */
pub const JET_errRedoAbruptEnded: JET_ERR=				-536;  /* Redo abruptly ended due to sudden failure in reading logs from log file */
pub const JET_errPatchFileMissing: JET_ERR=				-538;  /* Hard restore detected that patch file is missing from backup set */
pub const JET_errDatabaseLogSetMismatch: JET_ERR=		-539;  /* Database does not belong with the current set of log files */
pub const JET_errDatabaseStreamingFileMismatch: JET_ERR=	-540; /* Database and streaming file do not match each other */
pub const JET_errLogFileSizeMismatch: JET_ERR=			-541;  /* actual log file size does not match JET_paramLogFileSize */
pub const JET_errCheckpointFileNotFound: JET_ERR=		-542;  /* Could not locate checkpoint file */
pub const JET_errRequiredLogFilesMissing: JET_ERR=		-543;  /* The required log files for recovery is missing. */
pub const JET_errSoftRecoveryOnBackupDatabase: JET_ERR=	-544;  /* Soft recovery is intended on a backup database. Restore should be used instead */
pub const JET_errLogFileSizeMismatchDatabasesConsistent: JET_ERR=	-545;  /* databases have been recovered, but the log file size used during recovery does not match JET_paramLogFileSize */
pub const JET_errLogSectorSizeMismatch: JET_ERR=		-546;  /* the log file sector size does not match the current volume's sector size */
pub const JET_errLogSectorSizeMismatchDatabasesConsistent: JET_ERR=	-547;  /* databases have been recovered, but the log file sector size (used during recovery) does not match the current volume's sector size */
pub const JET_errLogSequenceEndDatabasesConsistent: JET_ERR=		-548; /* databases have been recovered, but all possible log generations in the current sequence are used; delete all log files and the checkpoint file and backup the databases before continuing */

pub const JET_errStreamingDataNotLogged: JET_ERR=		-549;  /* Illegal attempt to replay a streaming file operation where the data wasn't logged. Probably caused by an attempt to roll-forward with circular logging enabled */

pub const JET_errDatabaseDirtyShutdown: JET_ERR=		-550;  /* Database was not shutdown cleanly. Recovery must first be run to properly complete database operations for the previous shutdown. */
pub const JET_errDatabaseInconsistent: JET_ERR=			JET_errDatabaseDirtyShutdown;	/* OBSOLETE */
pub const JET_errConsistentTimeMismatch: JET_ERR=		-551;  /* Database last consistent time unmatched */
pub const JET_errDatabasePatchFileMismatch: JET_ERR=	-552;  /* Patch file is not generated from this backup */
pub const JET_errEndingRestoreLogTooLow: JET_ERR=		-553;  /* The starting log number too low for the restore */
pub const JET_errStartingRestoreLogTooHigh: JET_ERR=	-554;  /* The starting log number too high for the restore */
pub const JET_errGivenLogFileHasBadSignature: JET_ERR=	-555;  /* Restore log file has bad signature */
pub const JET_errGivenLogFileIsNotContiguous: JET_ERR=	-556;  /* Restore log file is not contiguous */
pub const JET_errMissingRestoreLogFiles: JET_ERR=		-557;  /* Some restore log files are missing */
pub const JET_wrnExistingLogFileHasBadSignature: JET_ERR=	558;  /* Existing log file has bad signature */
pub const JET_wrnExistingLogFileIsNotContiguous: JET_ERR=	559;  /* Existing log file is not contiguous */
pub const JET_errMissingFullBackup: JET_ERR=			-560;  /* The database missed a previous full backup before incremental backup */
pub const JET_errBadBackupDatabaseSize: JET_ERR=		-561;  /* The backup database size is not in 4k */
pub const JET_errDatabaseAlreadyUpgraded: JET_ERR=		-562;  /* Attempted to upgrade a database that is already current */
pub const JET_errDatabaseIncompleteUpgrade: JET_ERR=	-563;  /* Attempted to use a database which was only partially converted to the current format -- must restore from backup */
pub const JET_wrnSkipThisRecord: JET_ERR=				 564;  /* INTERNAL ERROR */
pub const JET_errMissingCurrentLogFiles: JET_ERR=		-565;  /* Some current log files are missing for continuous restore */

pub const JET_errDbTimeTooOld: JET_ERR=						-566;  /* dbtime on page smaller than dbtimeBefore in record */
pub const JET_errDbTimeTooNew: JET_ERR=						-567;  /* dbtime on page in advance of the dbtimeBefore in record */
pub const JET_errMissingFileToBackup: JET_ERR=				-569;  /* Some log or patch files are missing during backup */

pub const JET_errLogTornWriteDuringHardRestore: JET_ERR=	-570;	/* torn-write was detected in a backup set during hard restore */
pub const JET_errLogTornWriteDuringHardRecovery: JET_ERR=	-571;	/* torn-write was detected during hard recovery (log was not part of a backup set) */
pub const JET_errLogCorruptDuringHardRestore: JET_ERR=		-573;	/* corruption was detected in a backup set during hard restore */
pub const JET_errLogCorruptDuringHardRecovery: JET_ERR=	 	-574;	/* corruption was detected during hard recovery (log was not part of a backup set) */

pub const JET_errMustDisableLoggingForDbUpgrade: JET_ERR=	-575;	/* Cannot have logging enabled while attempting to upgrade db */

pub const JET_errBadRestoreTargetInstance: JET_ERR=			-577;	/* TargetInstance specified for restore is not found or log files don't match */
pub const JET_wrnTargetInstanceRunning: JET_ERR=			 578;	/* TargetInstance specified for restore is running */

pub const JET_errRecoveredWithoutUndo: JET_ERR=				-579;	/* Soft recovery successfully replayed all operations, but the Undo phase of recovery was skipped */

pub const JET_errDatabasesNotFromSameSnapshot: JET_ERR=		-580;	/* Databases to be restored are not from the same shadow copy backup */
pub const JET_errSoftRecoveryOnSnapshot: JET_ERR=			-581;	/* Soft recovery on a database from a shadow copy backup set */
pub const JET_errCommittedLogFilesMissing: JET_ERR=			-582;	/* One or more logs that were committed to this database, are missing.  These log files are required to maintain durable ACID semantics, but not required to maintain consistency if the JET_bitReplayIgnoreLostLogs bit is specified during recovery. */
pub const JET_errSectorSizeNotSupported: JET_ERR=			-583;	/* The physical sector size reported by the disk subsystem, is unsupported by ESE for a specific file type. */
pub const JET_errRecoveredWithoutUndoDatabasesConsistent: JET_ERR=	-584;	/* Soft recovery successfully replayed all operations and intended to skip the Undo phase of recovery, but the Undo phase was not required */
pub const JET_wrnCommittedLogFilesLost: JET_ERR=			585;		/* One or more logs that were committed to this database, were not recovered.  The database is still clean/consistent, as though the lost log's transactions were committed lazily (and lost). */
pub const JET_errCommittedLogFileCorrupt: JET_ERR=			-586;	/* One or more logs were found to be corrupt during recovery.  These log files are required to maintain durable ACID semantics, but not required to maintain consistency if the JET_bitIgnoreLostLogs bit and JET_paramDeleteOutOfRangeLogs is specified during recovery. */
pub const JET_wrnCommittedLogFilesRemoved: JET_ERR=			587;		/* One or more logs that were committed to this database, were no recovered.  The database is still clean/consistent, as though the corrupted log's transactions were committed lazily (and lost). */
pub const JET_wrnFinishWithUndo: JET_ERR=					588;		/* Signal used by clients to indicate JetInit() finished with undo */

pub const JET_wrnDatabaseRepaired: JET_ERR=					 595;	/* Database corruption has been repaired */

pub const JET_errUnicodeTranslationBufferTooSmall: JET_ERR=	-601;	/* Unicode translation buffer too small */
pub const JET_errUnicodeTranslationFail: JET_ERR=			-602;	/* Unicode normalization failed */
pub const JET_errUnicodeNormalizationNotSupported: JET_ERR=	-603;	/* OS does not provide support for Unicode normalisation (and no normalisation callback was specified) */
pub const JET_errUnicodeLanguageValidationFailure: JET_ERR=	-604;	/* Can not validate the language */

pub const JET_errExistingLogFileHasBadSignature: JET_ERR=	-610;	/* Existing log file has bad signature */
pub const JET_errExistingLogFileIsNotContiguous: JET_ERR=	-611;	/* Existing log file is not contiguous */

pub const JET_errLogReadVerifyFailure: JET_ERR=			-612;  /* Checksum error in log file during backup */

pub const JET_errCheckpointDepthTooDeep: JET_ERR=		-614;	//	too many outstanding generations between checkpoint and current generation

pub const JET_errRestoreOfNonBackupDatabase: JET_ERR=	-615;	//	hard recovery attempted on a database that wasn't a backup database
pub const JET_errLogFileNotCopied: JET_ERR=				-616;	//	log truncation attempted but not all required logs were copied
pub const JET_errTransactionTooLong: JET_ERR=			-618;	//	Too many outstanding generations between JetBeginTransaction and current generation.

pub const JET_errEngineFormatVersionNoLongerSupportedTooLow: JET_ERR=			-619; /* The specified JET_ENGINEFORMATVERSION value is too low to be supported by this version of ESE. */
pub const JET_errEngineFormatVersionNotYetImplementedTooHigh: JET_ERR=			-620; /* The specified JET_ENGINEFORMATVERSION value is too high, higher than this version of ESE knows about. */
pub const JET_errEngineFormatVersionParamTooLowForRequestedFeature: JET_ERR=	-621; /* Thrown by a format feature (not at JetSetSystemParameter) if the client requests a feature that requires a version higher than that set for the JET_paramEngineFormatVersion. */
pub const JET_errEngineFormatVersionSpecifiedTooLowForLogVersion: JET_ERR=						-622; /* The specified JET_ENGINEFORMATVERSION is set too low for this log stream, the log files have already been upgraded to a higher version.  A higher JET_ENGINEFORMATVERSION value must be set in the param. */
pub const JET_errEngineFormatVersionSpecifiedTooLowForDatabaseVersion: JET_ERR= 				-623; /* The specified JET_ENGINEFORMATVERSION is set too low for this database file, the database file has already been upgraded to a higher version.  A higher JET_ENGINEFORMATVERSION value must be set in the param. */

pub const JET_errBackupAbortByServer: JET_ERR=			-801;  /* Backup was aborted by server by calling JetTerm with JET_bitTermStopBackup or by calling JetStopBackup */

pub const JET_errInvalidGrbit: JET_ERR=					-900;  /* Invalid flags parameter */

pub const JET_errTermInProgress: JET_ERR=		  		-1000; /* Termination in progress */
pub const JET_errFeatureNotAvailable: JET_ERR=			-1001; /* API not supported */
pub const JET_errInvalidName: JET_ERR=					-1002; /* Invalid name */
pub const JET_errInvalidParameter: JET_ERR= 			-1003; /* Invalid API parameter */
pub const JET_wrnColumnNull: JET_ERR=					 1004; /* Column is NULL-valued */
pub const JET_wrnBufferTruncated: JET_ERR=				 1006; /* Buffer too small for data */
pub const JET_wrnDatabaseAttached: JET_ERR= 			 1007; /* Database is already attached */
pub const JET_errDatabaseFileReadOnly: JET_ERR=			-1008; /* Tried to attach a read-only database file for read/write operations */
pub const JET_wrnSortOverflow: JET_ERR=					 1009; /* Sort does not fit in memory */
pub const JET_errInvalidDatabaseId: JET_ERR=			-1010; /* Invalid database id */
pub const JET_errOutOfMemory: JET_ERR=					-1011; /* Out of Memory */
pub const JET_errOutOfDatabaseSpace: JET_ERR= 			-1012; /* Maximum database size reached */
pub const JET_errOutOfCursors: JET_ERR=					-1013; /* Out of table cursors */
pub const JET_errOutOfBuffers: JET_ERR=					-1014; /* Out of database page buffers */
pub const JET_errTooManyIndexes: JET_ERR=				-1015; /* Too many indexes */
pub const JET_errTooManyKeys: JET_ERR=					-1016; /* Too many columns in an index */
pub const JET_errRecordDeleted: JET_ERR=				-1017; /* Record has been deleted */
pub const JET_errReadVerifyFailure: JET_ERR=			-1018; /* Checksum error on a database page */
pub const JET_errPageNotInitialized: JET_ERR=			-1019; /* Blank database page */
pub const JET_errOutOfFileHandles: JET_ERR=	 			-1020; /* Out of file handles */
pub const JET_errDiskReadVerificationFailure: JET_ERR=	-1021; /* The OS returned ERROR_CRC from file IO */
pub const JET_errDiskIO: JET_ERR=						-1022; /* Disk IO error */
pub const JET_errInvalidPath: JET_ERR=					-1023; /* Invalid file path */
pub const JET_errInvalidSystemPath: JET_ERR=			-1024; /* Invalid system path */
pub const JET_errInvalidLogDirectory: JET_ERR=			-1025; /* Invalid log directory */
pub const JET_errRecordTooBig: JET_ERR=					-1026; /* Record larger than maximum size */
pub const JET_errTooManyOpenDatabases: JET_ERR=			-1027; /* Too many open databases */
pub const JET_errInvalidDatabase: JET_ERR=				-1028; /* Not a database file */
pub const JET_errNotInitialized: JET_ERR=				-1029; /* Database engine not initialized */
pub const JET_errAlreadyInitialized: JET_ERR=			-1030; /* Database engine already initialized */
pub const JET_errInitInProgress: JET_ERR=				-1031; /* Database engine is being initialized */
pub const JET_errFileAccessDenied: JET_ERR= 			-1032; /* Cannot access file, the file is locked or in use */
pub const JET_errBufferTooSmall: JET_ERR=				-1038; /* Buffer is too small */
pub const JET_wrnSeekNotEqual: JET_ERR=					 1039; /* Exact match not found during seek */
pub const JET_errTooManyColumns: JET_ERR=				-1040; /* Too many columns defined */
pub const JET_errContainerNotEmpty: JET_ERR=			-1043; /* Container is not empty */
pub const JET_errInvalidFilename: JET_ERR=				-1044; /* Filename is invalid */
pub const JET_errInvalidBookmark: JET_ERR=				-1045; /* Invalid bookmark */
pub const JET_errColumnInUse: JET_ERR=					-1046; /* Column used in an index */
pub const JET_errInvalidBufferSize: JET_ERR=			-1047; /* Data buffer doesn't match column size */
pub const JET_errColumnNotUpdatable: JET_ERR=			-1048; /* Cannot set column value */
pub const JET_errIndexInUse: JET_ERR=					-1051; /* Index is in use */
pub const JET_errLinkNotSupported: JET_ERR= 			-1052; /* Link support unavailable */
pub const JET_errNullKeyDisallowed: JET_ERR=			-1053; /* Null keys are disallowed on index */
pub const JET_errNotInTransaction: JET_ERR= 			-1054; /* Operation must be within a transaction */
pub const JET_wrnNoErrorInfo: JET_ERR=					 1055; /* No extended error information */
pub const JET_errMustRollback: JET_ERR=					-1057; /* Transaction must rollback because failure of unversioned update */
pub const JET_wrnNoIdleActivity: JET_ERR=		 		 1058; /* No idle activity occured */
pub const JET_errTooManyActiveUsers: JET_ERR=			-1059; /* Too many active database users */
pub const JET_errInvalidCountry: JET_ERR=				-1061; /* Invalid or unknown country/region code */
pub const JET_errInvalidLanguageId: JET_ERR=			-1062; /* Invalid or unknown language id */
pub const JET_errInvalidCodePage: JET_ERR=				-1063; /* Invalid or unknown code page */
pub const JET_errInvalidLCMapStringFlags: JET_ERR=		-1064; /* Invalid flags for LCMapString() */
pub const JET_errVersionStoreEntryTooBig: JET_ERR=		-1065; /* Attempted to create a version store entry (RCE) larger than a version bucket */
pub const JET_errVersionStoreOutOfMemoryAndCleanupTimedOut: JET_ERR=	-1066; /* Version store out of memory (and cleanup attempt failed to complete) */
pub const JET_wrnNoWriteLock: JET_ERR=					 1067; /* No write lock at transaction level 0 */
pub const JET_wrnColumnSetNull: JET_ERR=		   		 1068; /* Column set to NULL-value */
pub const JET_errVersionStoreOutOfMemory: JET_ERR=		-1069; /* Version store out of memory (cleanup already attempted) */
pub const JET_errCannotIndex: JET_ERR=		 	  		-1071; /* Cannot index escrow column */
pub const JET_errRecordNotDeleted: JET_ERR=				-1072; /* Record has not been deleted */
pub const JET_errTooManyMempoolEntries: JET_ERR=		-1073; /* Too many mempool entries requested */
pub const JET_errOutOfObjectIDs: JET_ERR=				-1074; /* Out of btree ObjectIDs (perform offline defrag to reclaim freed/unused ObjectIds) */
pub const JET_errOutOfLongValueIDs: JET_ERR=			-1075; /* Long-value ID counter has reached maximum value. (perform offline defrag to reclaim free/unused LongValueIDs) */
pub const JET_errOutOfAutoincrementValues: JET_ERR=		-1076; /* Auto-increment counter has reached maximum value (offline defrag WILL NOT be able to reclaim free/unused Auto-increment values). */
pub const JET_errOutOfDbtimeValues: JET_ERR=			-1077; /* Dbtime counter has reached maximum value (perform offline defrag to reclaim free/unused Dbtime values) */
pub const JET_errOutOfSequentialIndexValues: JET_ERR=	-1078; /* Sequential index counter has reached maximum value (perform offline defrag to reclaim free/unused SequentialIndex values) */

pub const JET_errRunningInOneInstanceMode: JET_ERR=		-1080; /* Multi-instance call with single-instance mode enabled */
pub const JET_errRunningInMultiInstanceMode: JET_ERR=	-1081; /* Single-instance call with multi-instance mode enabled */
pub const JET_errSystemParamsAlreadySet: JET_ERR=		-1082; /* Global system parameters have already been set */

pub const JET_errSystemPathInUse: JET_ERR=				-1083; /* System path already used by another database instance */
pub const JET_errLogFilePathInUse: JET_ERR=				-1084; /* Logfile path already used by another database instance */
pub const JET_errTempPathInUse: JET_ERR=				-1085; /* Temp path already used by another database instance */
pub const JET_errInstanceNameInUse: JET_ERR=			-1086; /* Instance Name already in use */
pub const JET_errSystemParameterConflict: JET_ERR=		-1087; /* Global system parameters have already been set, but to a conflicting or disagreeable state to the specified values. */

pub const JET_errInstanceUnavailable: JET_ERR=			-1090; /* This instance cannot be used because it encountered a fatal error */
pub const JET_errDatabaseUnavailable: JET_ERR=			-1091; /* This database cannot be used because it encountered a fatal error */
pub const JET_errInstanceUnavailableDueToFatalLogDiskFull: JET_ERR=	-1092; /* This instance cannot be used because it encountered a log-disk-full error performing an operation (likely transaction rollback) that could not tolerate failure */
pub const JET_errInvalidSesparamId: JET_ERR=			-1093; /* This JET_sesparam* identifier is not known to the ESE engine. */

pub const JET_errOutOfSessions: JET_ERR=  				-1101; /* Out of sessions */
pub const JET_errWriteConflict: JET_ERR=				-1102; /* Write lock failed due to outstanding write lock */
pub const JET_errTransTooDeep: JET_ERR=					-1103; /* Transactions nested too deeply */
pub const JET_errInvalidSesid: JET_ERR=					-1104; /* Invalid session handle */
pub const JET_errWriteConflictPrimaryIndex: JET_ERR=	-1105; /* Update attempted on uncommitted primary index */
pub const JET_errInTransaction: JET_ERR=				-1108; /* Operation not allowed within a transaction */
pub const JET_errRollbackRequired: JET_ERR=				-1109; /* Must rollback current transaction -- cannot commit or begin a new one */
pub const JET_errTransReadOnly: JET_ERR=				-1110; /* Read-only transaction tried to modify the database */
pub const JET_errSessionWriteConflict: JET_ERR=			-1111; /* Attempt to replace the same record by two diffrerent cursors in the same session */

pub const JET_errRecordTooBigForBackwardCompatibility: JET_ERR=				-1112; /* record would be too big if represented in a database format from a previous version of Jet */
pub const JET_errCannotMaterializeForwardOnlySort: JET_ERR=					-1113; /* The temp table could not be created due to parameters that conflict with JET_bitTTForwardOnly */

pub const JET_errSesidTableIdMismatch: JET_ERR=			-1114; /* This session handle can't be used with this table id */
pub const JET_errInvalidInstance: JET_ERR=				-1115; /* Invalid instance handle */
pub const JET_errDirtyShutdown: JET_ERR=				-1116; /* The instance was shutdown successfully but all the attached databases were left in a dirty state by request via JET_bitTermDirty */
// unused -1117
pub const JET_errReadPgnoVerifyFailure: JET_ERR=		-1118; /* The database page read from disk had the wrong page number. */
pub const JET_errReadLostFlushVerifyFailure: JET_ERR=	-1119; /* The database page read from disk had a previous write not represented on the page. */
pub const JET_errFileSystemCorruption: JET_ERR=				-1121; /* File system operation failed with an error indicating the file system is corrupt. */
pub const JET_wrnShrinkNotPossible: JET_ERR=				1122; /* Database file could not be shrunk because there is not enough internal free space available or there is unmovable data present. */
pub const JET_errRecoveryVerifyFailure: JET_ERR=			-1123; /* One or more database pages read from disk during recovery do not match the expected state. */

pub const JET_errFilteredMoveNotSupported: JET_ERR=			-1124; /* Attempted to provide a filter to JetSetCursorFilter() in an unsupported scenario. */

pub const JET_errDatabaseDuplicate: JET_ERR=			-1201; /* Database already exists */
pub const JET_errDatabaseInUse: JET_ERR=				-1202; /* Database in use */
pub const JET_errDatabaseNotFound: JET_ERR= 			-1203; /* No such database */
pub const JET_errDatabaseInvalidName: JET_ERR=			-1204; /* Invalid database name */
pub const JET_errDatabaseInvalidPages: JET_ERR=			-1205; /* Invalid number of pages */
pub const JET_errDatabaseCorrupted: JET_ERR=			-1206; /* Non database file or corrupted db */
pub const JET_errDatabaseLocked: JET_ERR=				-1207; /* Database exclusively locked */
pub const JET_errCannotDisableVersioning: JET_ERR=		-1208; /* Cannot disable versioning for this database */
pub const JET_errInvalidDatabaseVersion: JET_ERR=		-1209; /* Database engine is incompatible with database */

/*	The following error code are for NT clients only. It will return such error during
 *	JetInit if JET_paramCheckFormatWhenOpenFail is set.
 */
pub const JET_errDatabase200Format: JET_ERR=			-1210; /* The database is in an older (200) format */
pub const JET_errDatabase400Format: JET_ERR=			-1211; /* The database is in an older (400) format */
pub const JET_errDatabase500Format: JET_ERR=			-1212; /* The database is in an older (500) format */

pub const JET_errPageSizeMismatch: JET_ERR=				-1213; /* The database page size does not match the engine */
pub const JET_errTooManyInstances: JET_ERR=				-1214; /* Cannot start any more database instances */
pub const JET_errDatabaseSharingViolation: JET_ERR=		-1215; /* A different database instance is using this database */
pub const JET_errAttachedDatabaseMismatch: JET_ERR=		-1216; /* An outstanding database attachment has been detected at the start or end of recovery, but database is missing or does not match attachment info */
pub const JET_errDatabaseInvalidPath: JET_ERR=			-1217; /* Specified path to database file is illegal */
pub const JET_errDatabaseIdInUse: JET_ERR=				-1218; /* A database is being assigned an id already in use */
pub const JET_errForceDetachNotAllowed: JET_ERR= 		-1219; /* Force Detach allowed only after normal detach errored out */
pub const JET_errCatalogCorrupted: JET_ERR=				-1220; /* Corruption detected in catalog */
pub const JET_errPartiallyAttachedDB: JET_ERR=			-1221; /* Database is partially attached. Cannot complete attach operation */
pub const JET_errDatabaseSignInUse: JET_ERR=			-1222; /* Database with same signature in use */

pub const JET_errDatabaseCorruptedNoRepair: JET_ERR=	-1224; /* Corrupted db but repair not allowed */
pub const JET_errInvalidCreateDbVersion: JET_ERR=		-1225; /* recovery tried to replay a database creation, but the database was originally created with an incompatible (likely older) version of the database engine */

pub const JET_errDatabaseNotReady: JET_ERR=				-1230; /* Recovery on this database has not yet completed enough to permit access. */
pub const JET_errDatabaseAttachedForRecovery: JET_ERR=	-1231; /* Database is attached but only for recovery.  It must be explicitly attached before it can be opened.  */
pub const JET_errTransactionsNotReadyDuringRecovery: JET_ERR= -1232;  /* Recovery has not seen any Begin0/Commit0 records and so does not know what trxBegin0 to assign to this transaction */

pub const JET_wrnTableEmpty: JET_ERR=			 		 1301; /* Opened an empty table */
pub const JET_errTableLocked: JET_ERR=					-1302; /* Table is exclusively locked */
pub const JET_errTableDuplicate: JET_ERR=				-1303; /* Table already exists */
pub const JET_errTableInUse: JET_ERR=					-1304; /* Table is in use, cannot lock */
pub const JET_errObjectNotFound: JET_ERR=				-1305; /* No such table or object */
pub const JET_errDensityInvalid: JET_ERR=				-1307; /* Bad file/index density */
pub const JET_errTableNotEmpty: JET_ERR=				-1308; /* Table is not empty */
pub const JET_errInvalidTableId: JET_ERR=				-1310; /* Invalid table id */
pub const JET_errTooManyOpenTables: JET_ERR=			-1311; /* Cannot open any more tables (cleanup already attempted) */
pub const JET_errIllegalOperation: JET_ERR= 			-1312; /* Oper. not supported on table */
pub const JET_errTooManyOpenTablesAndCleanupTimedOut: JET_ERR=	-1313; /* Cannot open any more tables (cleanup attempt failed to complete) */
pub const JET_errObjectDuplicate: JET_ERR=				-1314; /* Table or object name in use */
pub const JET_errInvalidObject: JET_ERR=				-1316; /* Object is invalid for operation */
pub const JET_errCannotDeleteTempTable: JET_ERR=		-1317; /* Use CloseTable instead of DeleteTable to delete temp table */
pub const JET_errCannotDeleteSystemTable: JET_ERR=		-1318; /* Illegal attempt to delete a system table */
pub const JET_errCannotDeleteTemplateTable: JET_ERR=	-1319; /* Illegal attempt to delete a template table */
pub const JET_errExclusiveTableLockRequired: JET_ERR=	-1322; /* Must have exclusive lock on table. */
pub const JET_errFixedDDL: JET_ERR=						-1323; /* DDL operations prohibited on this table */
pub const JET_errFixedInheritedDDL: JET_ERR=			-1324; /* On a derived table, DDL operations are prohibited on inherited portion of DDL */
pub const JET_errCannotNestDDL: JET_ERR=				-1325; /* Nesting of hierarchical DDL is not currently supported. */
pub const JET_errDDLNotInheritable: JET_ERR=			-1326; /* Tried to inherit DDL from a table not marked as a template table. */
pub const JET_wrnTableInUseBySystem: JET_ERR=			 1327; /* System cleanup has a cursor open on the table */
pub const JET_errInvalidSettings: JET_ERR=				-1328; /* System parameters were set improperly */
pub const JET_errClientRequestToStopJetService: JET_ERR=			-1329;	/* Client has requested stop service */
pub const JET_errCannotAddFixedVarColumnToDerivedTable: JET_ERR=	-1330;	/* Template table was created with NoFixedVarColumnsInDerivedTables */

//	DDL errors
// Note: Some DDL errors have snuck into other categories.
pub const JET_errIndexCantBuild: JET_ERR=				-1401; /* Index build failed */
pub const JET_errIndexHasPrimary: JET_ERR=				-1402; /* Primary index already defined */
pub const JET_errIndexDuplicate: JET_ERR=				-1403; /* Index is already defined */
pub const JET_errIndexNotFound: JET_ERR=				-1404; /* No such index */
pub const JET_errIndexMustStay: JET_ERR=				-1405; /* Cannot delete clustered index */
pub const JET_errIndexInvalidDef: JET_ERR=				-1406; /* Illegal index definition */
pub const JET_errInvalidCreateIndex: JET_ERR=	 		-1409; /* Invalid create index description */
pub const JET_errTooManyOpenIndexes: JET_ERR=			-1410; /* Out of index description blocks */
pub const JET_errMultiValuedIndexViolation: JET_ERR=	-1411; /* Non-unique inter-record index keys generated for a multivalued index */
pub const JET_errIndexBuildCorrupted: JET_ERR=			-1412; /* Failed to build a secondary index that properly reflects primary index */
pub const JET_errPrimaryIndexCorrupted: JET_ERR=		-1413; /* Primary index is corrupt. The database must be defragmented or the table deleted. */
pub const JET_errSecondaryIndexCorrupted: JET_ERR=		-1414; /* Secondary index is corrupt. The database must be defragmented or the affected index must be deleted. If the corrupt index is over Unicode text, a likely cause a sort-order change. */
pub const JET_wrnCorruptIndexDeleted: JET_ERR=			 1415; /* Out of date index removed */
pub const JET_errInvalidIndexId: JET_ERR=				-1416; /* Illegal index id */
pub const JET_wrnPrimaryIndexOutOfDate: JET_ERR=		 1417; /* The Primary index is created with an incompatible OS sort version. The table can not be safely modified. */
pub const JET_wrnSecondaryIndexOutOfDate: JET_ERR=		 1418; /* One or more Secondary index is created with an incompatible OS sort version. Any index over Unicode text should be deleted. */

pub const JET_errIndexTuplesSecondaryIndexOnly: JET_ERR=		-1430;	//	tuple index can only be on a secondary index
pub const JET_errIndexTuplesTooManyColumns: JET_ERR=			-1431;	//	tuple index may only have eleven columns in the index
pub const JET_errIndexTuplesOneColumnOnly: JET_ERR=				JET_errIndexTuplesTooManyColumns;	/* OBSOLETE */
pub const JET_errIndexTuplesNonUniqueOnly: JET_ERR=				-1432;	//	tuple index must be a non-unique index
pub const JET_errIndexTuplesTextBinaryColumnsOnly: JET_ERR=		-1433;	//	tuple index must be on a text/binary column
pub const JET_errIndexTuplesTextColumnsOnly: JET_ERR=			JET_errIndexTuplesTextBinaryColumnsOnly;		/* OBSOLETE */
pub const JET_errIndexTuplesVarSegMacNotAllowed: JET_ERR=		-1434;	//	tuple index does not allow setting cbVarSegMac
pub const JET_errIndexTuplesInvalidLimits: JET_ERR=				-1435;	//	invalid min/max tuple length or max characters to index specified
pub const JET_errIndexTuplesCannotRetrieveFromIndex: JET_ERR=	-1436;	//	cannot call RetrieveColumn() with RetrieveFromIndex on a tuple index
pub const JET_errIndexTuplesKeyTooSmall: JET_ERR=				-1437;	//	specified key does not meet minimum tuple length
pub const JET_errInvalidLVChunkSize: JET_ERR=					-1438;	//	Specified LV chunk size is not supported
pub const JET_errColumnCannotBeEncrypted: JET_ERR=				-1439;	//	Only JET_coltypLongText and JET_coltypLongBinary columns can be encrypted
pub const JET_errCannotIndexOnEncryptedColumn: JET_ERR=			-1440;	//	Cannot index encrypted column

//	DML errors
// Note: Some DML errors have snuck into other categories.
// Note: Some DDL errors have inappropriately snuck in here.
pub const JET_errColumnLong: JET_ERR=					-1501; /* Column value is long */
pub const JET_errColumnNoChunk: JET_ERR=				-1502; /* No such chunk in long value */
pub const JET_errColumnDoesNotFit: JET_ERR= 			-1503; /* Field will not fit in record */
pub const JET_errNullInvalid: JET_ERR=					-1504; /* Null not valid */
pub const JET_errColumnIndexed: JET_ERR=				-1505; /* Column indexed, cannot delete */
pub const JET_errColumnTooBig: JET_ERR=					-1506; /* Field length is greater than maximum */
pub const JET_errColumnNotFound: JET_ERR=				-1507; /* No such column */
pub const JET_errColumnDuplicate: JET_ERR=				-1508; /* Field is already defined */
pub const JET_errMultiValuedColumnMustBeTagged: JET_ERR=	-1509; /* Attempted to create a multi-valued column, but column was not Tagged */
pub const JET_errColumnRedundant: JET_ERR=				-1510; /* Second autoincrement or version column */
pub const JET_errInvalidColumnType: JET_ERR=			-1511; /* Invalid column data type */
pub const JET_wrnColumnMaxTruncated: JET_ERR=	 		 1512; /* Max length too big, truncated */
pub const JET_errTaggedNotNULL: JET_ERR=				-1514; /* No non-NULL tagged columns */
pub const JET_errNoCurrentIndex: JET_ERR=				-1515; /* Invalid w/o a current index */
pub const JET_errKeyIsMade: JET_ERR=					-1516; /* The key is completely made */
pub const JET_errBadColumnId: JET_ERR=					-1517; /* Column Id Incorrect */
pub const JET_errBadItagSequence: JET_ERR=				-1518; /* Bad itagSequence for tagged column */
pub const JET_errColumnInRelationship: JET_ERR=			-1519; /* Cannot delete, column participates in relationship */
pub const JET_wrnCopyLongValue: JET_ERR=				 1520; /* Single instance column bursted */
pub const JET_errCannotBeTagged: JET_ERR=				-1521; /* AutoIncrement and Version cannot be tagged */
pub const JET_errDefaultValueTooBig: JET_ERR=			-1524; /* Default value exceeds maximum size */
pub const JET_errMultiValuedDuplicate: JET_ERR=			-1525; /* Duplicate detected on a unique multi-valued column */
pub const JET_errLVCorrupted: JET_ERR=					-1526; /* Corruption encountered in long-value tree */
pub const JET_errMultiValuedDuplicateAfterTruncation: JET_ERR=	-1528; /* Duplicate detected on a unique multi-valued column after data was normalized, and normalizing truncated the data before comparison */
pub const JET_errDerivedColumnCorruption: JET_ERR=		-1529; /* Invalid column in derived table */
pub const JET_errInvalidPlaceholderColumn: JET_ERR=		-1530; /* Tried to convert column to a primary index placeholder, but column doesn't meet necessary criteria */
pub const JET_wrnColumnSkipped: JET_ERR=				 1531; /* Column value(s) not returned because the corresponding column id or itagSequence requested for enumeration was null */
pub const JET_wrnColumnNotLocal: JET_ERR=				 1532; /* Column value(s) not returned because they could not be reconstructed from the data at hand */
pub const JET_wrnColumnMoreTags: JET_ERR=				 1533; /* Column values exist that were not requested for enumeration */
pub const JET_wrnColumnTruncated: JET_ERR=				 1534; /* Column value truncated at the requested size limit during enumeration */
pub const JET_wrnColumnPresent: JET_ERR=				 1535; /* Column values exist but were not returned by request */
pub const JET_wrnColumnSingleValue: JET_ERR=			 1536; /* Column value returned in JET_COLUMNENUM as a result of JET_bitEnumerateCompressOutput */
pub const JET_wrnColumnDefault: JET_ERR=				 1537; /* Column value(s) not returned because they were set to their default value(s) and JET_bitEnumerateIgnoreDefault was specified */
pub const JET_errColumnCannotBeCompressed: JET_ERR=		-1538; /* Only JET_coltypLongText and JET_coltypLongBinary columns can be compressed */
pub const JET_wrnColumnNotInRecord: JET_ERR=			 1539; /* Column value(s) not returned because they could not be reconstructed from the data in the record */
pub const JET_errColumnNoEncryptionKey: JET_ERR=		-1540; /* Cannot retrieve/set encrypted column without an encryption key */

pub const JET_errRecordNotFound: JET_ERR=				-1601; /* The key was not found */
pub const JET_errRecordNoCopy: JET_ERR=					-1602; /* No working buffer */
pub const JET_errNoCurrentRecord: JET_ERR=				-1603; /* Currency not on a record */
pub const JET_errRecordPrimaryChanged: JET_ERR=			-1604; /* Primary key may not change */
pub const JET_errKeyDuplicate: JET_ERR=					-1605; /* Illegal duplicate key */
pub const JET_errAlreadyPrepared: JET_ERR=				-1607; /* Attempted to update record when record update was already in progress */
pub const JET_errKeyNotMade: JET_ERR=					-1608; /* No call to JetMakeKey */
pub const JET_errUpdateNotPrepared: JET_ERR=			-1609; /* No call to JetPrepareUpdate */
pub const JET_wrnDataHasChanged: JET_ERR=		 		 1610; /* Data has changed */
pub const JET_errDataHasChanged: JET_ERR=				-1611; /* Data has changed, operation aborted */
pub const JET_wrnKeyChanged: JET_ERR=			 		 1618; /* Moved to new key */
pub const JET_errLanguageNotSupported: JET_ERR=			-1619; /* Windows installation does not support language */
pub const JET_errDecompressionFailed: JET_ERR=			-1620; /* Internal error: data could not be decompressed */
pub const JET_errUpdateMustVersion: JET_ERR=			-1621; /* No version updates only for uncommitted tables */
pub const JET_errDecryptionFailed: JET_ERR=				-1622; /* Data could not be decrypted */

//	Sort Table errors
pub const JET_errTooManySorts: JET_ERR=					-1701; /* Too many sort processes */
pub const JET_errInvalidOnSort: JET_ERR=				-1702; /* Invalid operation on Sort */

//	Other errors
pub const JET_errTempFileOpenError: JET_ERR=			-1803; /* Temp file could not be opened */
pub const JET_errTooManyAttachedDatabases: JET_ERR= 	-1805; /* Too many open databases */
pub const JET_errDiskFull: JET_ERR= 					-1808; /* No space left on disk */
pub const JET_errPermissionDenied: JET_ERR= 			-1809; /* Permission denied */
pub const JET_errFileNotFound: JET_ERR=					-1811; /* File not found */
pub const JET_errFileInvalidType: JET_ERR=				-1812; /* Invalid file type */
pub const JET_wrnFileOpenReadOnly: JET_ERR=				 1813; /* Database file is read only */

pub const JET_errAfterInitialization: JET_ERR=			-1850; /* Cannot Restore after init. */
pub const JET_errLogCorrupted: JET_ERR=					-1852; /* Logs could not be interpreted */

pub const JET_errInvalidOperation: JET_ERR= 			-1906; /* Invalid operation */
pub const JET_errAccessDenied: JET_ERR=					-1907; /* Access denied */
pub const JET_wrnIdleFull: JET_ERR=						 1908; /* Idle registry full */
pub const JET_errTooManySplits: JET_ERR=				-1909; /* Infinite split */
pub const JET_errSessionSharingViolation: JET_ERR=		-1910; /* Multiple threads are using the same session */
pub const JET_errEntryPointNotFound: JET_ERR=			-1911; /* An entry point in a DLL we require could not be found */
pub const JET_errSessionContextAlreadySet: JET_ERR=		-1912; /* Specified session already has a session context set */
pub const JET_errSessionContextNotSetByThisThread: JET_ERR=	-1913; /* Tried to reset session context, but current thread did not orignally set the session context */
pub const JET_errSessionInUse: JET_ERR=					-1914; /* Tried to terminate session in use */
pub const JET_errRecordFormatConversionFailed: JET_ERR=	-1915; /* Internal error during dynamic record format conversion */
pub const JET_errOneDatabasePerSession: JET_ERR=		-1916; /* Just one open user database per session is allowed (JET_paramOneDatabasePerSession) */
pub const JET_errRollbackError: JET_ERR=				-1917; /* error during rollback */
pub const JET_errFlushMapVersionUnsupported: JET_ERR=	-1918; /* The version of the persisted flush map is not supported by this version of the engine. */
pub const JET_errFlushMapDatabaseMismatch: JET_ERR=		-1919; /* The persisted flush map and the database do not match. */
pub const JET_errFlushMapUnrecoverable: JET_ERR=		-1920; /* The persisted flush map cannot be reconstructed. */

pub const JET_wrnDefragAlreadyRunning: JET_ERR=			 2000; /* Online defrag already running on specified database */
pub const JET_wrnDefragNotRunning: JET_ERR=				 2001; /* Online defrag not running on specified database */
pub const JET_errDatabaseAlreadyRunningMaintenance: JET_ERR= -2004;	/* The operation did not complete successfully because the database is already running maintenance on specified database */

pub const JET_wrnCallbackNotRegistered: JET_ERR=         2100; /* Unregistered a non-existant callback function */
pub const JET_errCallbackFailed: JET_ERR=				-2101; /* A callback failed */
pub const JET_errCallbackNotResolved: JET_ERR=			-2102; /* A callback function could not be found */

pub const JET_errSpaceHintsInvalid: JET_ERR=			-2103; /* An element of the JET space hints structure was not correct or actionable. */

pub const JET_errOSSnapshotInvalidSequence: JET_ERR=	-2401; /* OS Shadow copy API used in an invalid sequence */
pub const JET_errOSSnapshotTimeOut: JET_ERR=			-2402; /* OS Shadow copy ended with time-out */
pub const JET_errOSSnapshotNotAllowed: JET_ERR=			-2403; /* OS Shadow copy not allowed (backup or recovery in progress) */
pub const JET_errOSSnapshotInvalidSnapId: JET_ERR=		-2404; /* invalid JET_OSSNAPID */

pub const JET_errLSCallbackNotSpecified: JET_ERR=		-3000; /* Attempted to use Local Storage without a callback function being specified */
pub const JET_errLSAlreadySet: JET_ERR=					-3001; /* Attempted to set Local Storage for an object which already had it set */
pub const JET_errLSNotSet: JET_ERR=						-3002; /* Attempted to retrieve Local Storage from an object which didn't have it set */

// FILE and DISK ERRORS
//JET_errFileAccessDenied					-1032
//JET_errFileNotFound						-1811
//JET_errInvalidFilename					-1044
pub const JET_errFileIOSparse: JET_ERR=					-4000; /* an I/O was issued to a location that was sparse */
pub const JET_errFileIOBeyondEOF: JET_ERR=				-4001; /* a read was issued to a location beyond EOF (writes will expand the file) */
pub const JET_errFileIOAbort: JET_ERR=					-4002; /* instructs the JET_ABORTRETRYFAILCALLBACK caller to abort the specified I/O */
pub const JET_errFileIORetry: JET_ERR=					-4003; /* instructs the JET_ABORTRETRYFAILCALLBACK caller to retry the specified I/O */
pub const JET_errFileIOFail: JET_ERR=					-4004; /* instructs the JET_ABORTRETRYFAILCALLBACK caller to fail the specified I/O */
pub const JET_errFileCompressed: JET_ERR=				-4005; /* read/write access is not supported on compressed files */
