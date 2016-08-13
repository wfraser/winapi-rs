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
