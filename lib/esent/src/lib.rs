// Copyright © 2015, William R. Fraser
// Licensed under the MIT License <LICENSE.md>
//! FFI bindings to esent.
#![no_std]
extern crate winapi;
use winapi::*;
extern "system" {
    pub fn JetAddColumnW(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        szColumnName: PCWSTR,
        pcolumndef: *const JET_COLUMNDEF,
        pvDefault: PCVOID,
        cbDefault: ULONG,
        pcolumnid: *mut JET_COLUMNID,
    ) -> JET_ERR;
    pub fn JetAttachDatabaseW(
        sesid: JET_SESID,
        szFilename: PCWSTR,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetAttachDatabase2W(
        sesid: JET_SESID,
        szFilename: PCWSTR,
        cpgDatabaseSizeMax: ULONG,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetBackupW(
        szBackupPath: PCWSTR,
        grbt: JET_GRBIT,
        pfnStatus: JET_PFNSTATUS,
    ) -> JET_ERR;
    pub fn JetBackupInstanceW(
        instance: JET_INSTANCE,
        szBackupPath: PCWSTR,
        grbit: JET_GRBIT,
        pfnStatus: JET_PFNSTATUS,
    ) -> JET_ERR;
    pub fn JetBeginExternalBackup(
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetBeginExternalBackupInstance(
        instance: JET_INSTANCE,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetBeginSessionW(
        instance: JET_INSTANCE,
        psesid: *mut JET_SESID,
        szUserName: PCWSTR,
        szPassword: PCWSTR,
    ) -> JET_ERR;
    pub fn JetBeginTransaction(
        sesid: JET_SESID,
    ) -> JET_ERR;
    pub fn JetBeginTransaction2(
        sesid: JET_SESID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetBeginTransaction3(
        sesid: JET_SESID,
        trxid: i64,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetCloseDatabase(
        sesid: JET_SESID,
        dbid: JET_DBID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetCloseFile(
        hfFile: JET_HANDLE,
    ) -> JET_ERR;
    pub fn JetCloseFileInstance(
        instance: JET_INSTANCE,
        hfFile: JET_HANDLE,
    ) -> JET_ERR;
    pub fn JetCloseTable(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
    ) -> JET_ERR;
    pub fn JetCommitTransaction(
        sesid: JET_SESID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetCommitTransaction2(
        sesid: JET_SESID,
        grbit: JET_GRBIT,
        cmsecDurableCommit: DWORD,
        pCommitId: JET_COMMIT_ID,
    ) -> JET_ERR;
    pub fn JetCompactW(
        sesid: JET_SESID,
        szDatabaseSrc: PCWSTR,
        szDatabaseDest: PCWSTR,
        pfnStatus: JET_PFNSTATUS,
        pconvert: PVOID, // actually *JET_CONVERT (but not supported since WinXP)
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetComputeStats(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
    ) -> JET_ERR;
    pub fn JetCreateDatabaseW(
        sesid: JET_SESID,
        szFilename: PCWSTR,
        szConnect: PCWSTR,
        pdbid: *mut JET_DBID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetCreateDatabase2W(
        sesid: JET_SESID,
        szFilename: PCWSTR,
        cpgDatabaseSizeMax: ULONG,
        pdbid: *mut JET_DBID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetCreateIndexW(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        szIndexName: PCWSTR,
        grbit: JET_GRBIT,
        szKey: PCWSTR,
        cbKey: ULONG,
        lDensity: ULONG,
    ) -> JET_ERR;
    pub fn JetCreateIndex2W(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        pindexcreate: *const JET_INDEXCREATE_W,
        cIndexCreate: ULONG,
    ) -> JET_ERR;
    pub fn JetCreateIndex3W(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        pindexcreate: *const JET_INDEXCREATE2_W,
        cIndexCreate: ULONG,
    ) -> JET_ERR;
    pub fn JetCreateIndex4W(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        pindexcreate: *const JET_INDEXCREATE3_W,
        cIndexCreate: ULONG,
    ) -> JET_ERR;
    pub fn JetCreateInstanceW(
        pinstance: *mut JET_INSTANCE,
        szInstanceName: PCWSTR,
    ) -> JET_ERR;
    pub fn JetCreateInstance2W(
        pinstance: *mut JET_INSTANCE,
        szInstanceName: PCWSTR,
        szDisplayName: PCWSTR,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetCreateTableW(
        sesid: JET_SESID,
        dbid: JET_DBID,
        szTableName: PCWSTR,
        lPages: ULONG,
        lDensity: ULONG,
        ptableid: *mut JET_TABLEID,
    ) -> JET_ERR;
    pub fn JetCreateTableColumnIndexW(
        sesid: JET_SESID,
        dbid: JET_DBID,
        ptablecreate: *mut JET_TABLECREATE_W,
    ) -> JET_ERR;
    // JetCreateTableColumnIndex2W
    // JetCreateTableColumnIndex3W
    pub fn JetCreateTableColumnIndex4W(
        sesid: JET_SESID,
        dbid: JET_DBID,
        ptablecreate: *mut JET_TABLECREATE3_W,
    ) -> JET_ERR;
    // JetDefragment
    // JetDefragment2
    // JetDefragment3
    pub fn JetDelete(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
    ) -> JET_ERR;
    pub fn JetDeleteColumnW(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        szColumnName: PCWSTR,
    ) -> JET_ERR;
    pub fn JetDeleteColumn2W(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        szColumnName: PCWSTR,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetDeleteIndex(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        szIndexName: PCWSTR,
    ) -> JET_ERR;
    pub fn JetDeleteTableW(
        sesid: JET_SESID,
        dbid: JET_DBID,
        szTableName: PCWSTR,
    ) -> JET_ERR;
    pub fn JetDetachDatabaseW(
        sesid: JET_SESID,
        szFilename: PCWSTR,
    ) -> JET_ERR;
    pub fn JetDetachDatabase2W(
        sesid: JET_SESID,
        szFilename: PCWSTR,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    // JetDupCursor
    // JetDupSession
    // JetEnableMultiInstance
    // JetEndExternalBackup
    // JetEndExternalBackupInstance
    // JetEndExternalBackupInstance2
    pub fn JetEndSession(
        sesid: JET_SESID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    // JetEnumerateColumns
    // JetEscrowUpdate
    // JetExternalRestore
    // JetExternalRestore2
    pub fn JetFreeBuffer(
        pbBuf: *mut u8,
    ) -> JET_ERR;
    // JetGetAttachInfo
    // JetGetAttachInfoInstance
    // JetGetBookmark
    pub fn JetGetColumnInfoW(
        sesid: JET_SESID,
        dbid: JET_DBID,
        szTableName: PCWSTR,
        pwColumnNameOrId: PCWSTR,
        pvResult: PVOID,
        cbMax: ULONG,
        InfoLevel: ULONG,
    ) -> JET_ERR;
    // JetGetCurrentIndex
    // JetGetCursorInfo
    // JetGetDatabaseFileInfo
    // JetGetDetabaseInfo
    pub fn JetGetErrorInfoW(
        pvContext: PCVOID,
        pvResult: PVOID,
        cbMax: ULONG,
        InfoLevel: ULONG,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    // JetGetIndexInfo
    // JetGetInstanceInfo
    // JetGetInstanceMiscInfo
    pub fn JetGetLock(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    // JetGetLogInfo
    // JetGetLogInfoInstance
    // JetGetLogInfoInstance2
    // JetGetLS
    // JetGetObjectInfo
    // JetGetRecordPosition
    // JetGetRecordSize
    // JetGetRecordSize2
    // JetGetSecondaryIndexBookmark
    // JetGetSessionParameter
    // JetGetSystemParameter
    pub fn JetGetTableColumnInfoW(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        szColumnName: PCWSTR,
        pvResult: PVOID,
        cbMax: ULONG,
        InfoLevel: ULONG,
    ) -> JET_ERR;
    // JetGetTableIndexInfo
    // JetGetTableInfo
    // JetGetThreadStats
    // JetGetTruncateLogInfoInstance
    pub fn JetGetVersion(
        sesid: JET_SESID,
        pwVersion: *mut ULONG,
    ) -> JET_ERR;
    // JetGotoBookmark
    // JetGotoPosition
    // JetGrowDatabase
    // JetIdle
    // JetIndexRecordCount
    pub fn JetInit(
        pinstance: *mut JET_INSTANCE,
    ) -> JET_ERR;
    pub fn JetInit2(
        pinstance: *mut JET_INSTANCE,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetInit3W(
        pinstance: *mut JET_INSTANCE,
        prstInfo: *const JET_RSTINFO_W,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    // JetIntersectIndexes
    // JetMakeKey
    pub fn JetMove(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        cRow: LONG,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetOpenDatabaseW(
        sesid: JET_SESID,
        szFilename: PCWSTR,
        szConnect: PCWSTR,
        pdbid: *mut JET_DBID,
        grbit: JET_GRBIT
    ) -> JET_ERR;
    // JetOpenFile
    // JetOpenFileInstance
    pub fn JetOpenTableW(
        sesid: JET_SESID,
        dbid: JET_DBID,
        szTableName: PCWSTR,
        pvParameters: PCVOID,
        cbParameters: ULONG,
        grbit: JET_GRBIT,
        ptableid: *mut JET_TABLEID,
    ) -> JET_ERR;
    // JetOpenTemporaryTable
    // JetOpenTempTable
    // JetOpenTempTable2
    // JetOpenTempTable3
    // JetOSSnapshotAbort
    // JetOSSnapshotEnd
    // JetOSSnapshotFreeze
    // JetOSSnapshotGetFreezeInfo
    // JetOSSnapshotPrepare
    // JetOSSnapshotPrepareInstance
    // JetOSSnapshotThaw
    // JetOSSnapshotTruncateLog
    // JetOSSnapshotTruncateLogInstance
    // JetPrepareUpdate
    // JetPrereadIndexRanges
    // JetPrereadKeys
    // JetReadFile
    // JetReadFileInstance
    // JetRegisterCallback
    // JetRenameColumn
    // JetRenameTable
    // JetResetSessionContext
    // JetResetTableSequential
    // JetResizeDatabase
    // JetRestore
    // JetRestore2
    // JetRestoreInstance
    pub fn JetRetrieveColumn(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        columnid: JET_COLUMNID,
        pvData: PVOID,
        cbData: ULONG,
        pcbActual: *mut ULONG,
        grbit: JET_GRBIT,
        pretinfo: *mut JET_RETINFO,
    ) -> JET_ERR;
    pub fn JetRetrieveColumns(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        pretrievecolumn: *mut JET_RETRIEVECOLUMN,
        cretrievecolumn: ULONG,
    ) -> JET_ERR;
    pub fn JetRetrieveKey(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        pvData: PVOID,
        cbMax: ULONG,
        pcbActual: *mut ULONG,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetRollback(
        sesid: JET_SESID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetSeek(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetSetColumn(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        columnid: JET_COLUMNID,
        pvData: PCVOID,
        cbData: ULONG,
        grbit: JET_GRBIT,
        psetinfo: *const JET_SETINFO,
    ) -> JET_ERR;
    pub fn JetSetColumnDefaultValueW(
        sesid: JET_SESID,
        dbid: JET_DBID,
        szTableName: PCWSTR,
        szColumnName: PCWSTR,
        pvData: PCVOID,
        cbData: ULONG,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetSetColumns(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        psetcolumn: *const JET_SETCOLUMN,
        csetcolumn: ULONG,
    ) -> JET_ERR;
    pub fn JetSetCurrentIndexW(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        szIndexName: PCWSTR,
    ) -> JET_ERR;
    pub fn JetSetCurrentIndex2W(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        szIndexName: PCWSTR,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetSetCurrentIndex3W(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        szIndexName: PCWSTR,
        grbit: JET_GRBIT,
        itagSequence: ULONG,
    ) -> JET_ERR;
    pub fn JetSetCurrentIndex4W(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        szIndexName: PCWSTR,
        pindexid: *const JET_INDEXID,
        grbit: JET_GRBIT,
        itagSequence: ULONG,
    ) -> JET_ERR;
    pub fn JetSetCursorFilter(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        rgFilters: *const JET_INDEX_COLUMN,
        cFilters: DWORD,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetSetDatabaseSizeW(
        sesid: JET_SESID,
        szDatabaseName: PCWSTR,
        cpg: ULONG,
        pcpgReal: *mut ULONG,
    ) -> JET_ERR;
    pub fn JetSetIndexRange(
        sesid: JET_SESID,
        tableidSrc: JET_TABLEID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetSetLS(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        ls: JET_LS,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetSetSessionContext(
        sesid: JET_SESID,
        ulContext: JET_API_PTR,
    ) -> JET_ERR;
    pub fn JetSetSessionParameter(
        sesid: JET_SESID,
        sesparamid: ULONG,
        pvParam: PCVOID,
        cbParam: ULONG,
    ) -> JET_ERR;
    pub fn JetSetSystemParameterW(
        pinstance: *mut JET_INSTANCE,
        sesid: JET_SESID,
        paramid: ULONG,
        lParam: JET_API_PTR,
        szParam: PCWSTR,
    ) -> JET_ERR;
    pub fn JetSetTableSequential(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetStopBackup() -> JET_ERR;
    pub fn JetStopBackupInstance(
        instance: JET_INSTANCE,
    ) -> JET_ERR;
    pub fn JetStopService() -> JET_ERR;
    pub fn JetStopServiceInstance(
        instance: JET_INSTANCE,
    ) -> JET_ERR;
    pub fn JetStopServiceInstance2(
        instance: JET_INSTANCE,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetTerm(
        instance: JET_INSTANCE,
    ) -> JET_ERR;
    pub fn JetTerm2(
        instance: JET_INSTANCE,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
    pub fn JetTruncateLog() -> JET_ERR;
    pub fn JetTruncateLogInstance(
        instance: JET_INSTANCE,
    ) -> JET_ERR;
    pub fn JetUnregisterCallback(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        cbtyp: JET_CBTYP,
        hCallbackId: JET_HANDLE,
    ) -> JET_ERR;
    pub fn JetUpdate(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        pvBookmark: PVOID,
        cbBookmark: ULONG,
        pcbActual: *mut ULONG,
    ) -> JET_ERR;
    pub fn JetUpdate2(
        sesid: JET_SESID,
        tableid: JET_TABLEID,
        pvBookmark: PVOID,
        cbBookmark: ULONG,
        pcbActual: *mut ULONG,
        grbit: JET_GRBIT,
    ) -> JET_ERR;
}
