(HexData) => 
{ 
    var arr = [{ id: 'Timestamp', dp: 'int32LE', sizeB: 4, multiplier: false }, { id: '_3V3', dp: 'Uint16LE', sizeB: 2, multiplier: 0.001220703125 }, { id: '_3V3RF', dp: 'Uint16LE', sizeB: 2, multiplier: 0.001220703125 }, { id: '_V5V', dp: 'Uint16LE', sizeB: 2, multiplier: 0.00244414062 }, { id: '_V_BAT', dp: 'Uint16LE', sizeB: 2, multiplier: 0.001220703125 }, { id: 'V_MPPT', dp: 'Uint16LE', sizeB: 2, multiplier: 0.001220703125 }, { id: 'Current_3V3', dp: 'Uint16LE', sizeB: 2, multiplier: 0.6103515625 }, { id: 'Current_3V3RF', dp: 'Uint16LE', sizeB: 2, multiplier: 0.6103515625 }, { id: 'Current_V5V', dp: 'Uint16LE', sizeB: 2, multiplier: 0.6103515625 }, { id: 'Current_MPPT', dp: 'Uint16LE', sizeB: 2, multiplier: 0.6103515625 }, { id: 'Current_VBAT', dp: 'Uint16LE', sizeB: 2, multiplier: 0.6103515625 }, { id: 'Battery_Temp', dp: 'FloatLE', sizeB: 4, multiplier: false }, { id: 'Battery_Temp_2', dp: 'FloatLE', sizeB: 4, multiplier: false }, { id: 'Temp_Solar_Panel_1', dp: 'FloatLE', sizeB: 4, multiplier: false }, { id: 'Temp_Solar_Panel_2', dp: 'FloatLE', sizeB: 4, multiplier: false }, { id: 'Temp_Solar_Panel_3', dp: 'FloatLE', sizeB: 4, multiplier: false }, { id: 'Temp_Solar_Panel_4', dp: 'FloatLE', sizeB: 4, multiplier: false }, { id: 'Fault_3V3_RF_V5V', dp: 'Uint8', sizeB: 1, multiplier: false }, { id: 'OBC_Mode', dp: 'Uint8', sizeB: 1, multiplier: false }, { id: 'Heater_Status', dp: 'Uint8', sizeB: 1, multiplier: false }, { id: 'Firmware_Version', dp: 'hex', sizeB: 1, multiplier: false }, { id: 'Last_Reception_Time', dp: 'int32LE', sizeB: 4, multiplier: false }, { id: 'Last_RSSI', dp: 'FloatLE', sizeB: 4, multiplier: false }, { id: 'S_F_Packet_Count', dp: 'Uint16LE', sizeB: 2, multiplier: false }, { id: 'Bad_RX_Count', dp: 'Uint16LE', sizeB: 2, multiplier: false }, { id: 'Last_Transmit_Time', dp: 'int32LE', sizeB: 4, multiplier: false }, { id: 'TX_Packets', dp: 'Uint16LE', sizeB: 2, multiplier: false }, { id: 'RX_Packets', dp: 'Uint16LE', sizeB: 2, multiplier: false }, { id: 'Beacon_Interval', dp: 'Uint8', sizeB: 1, multiplier: false }, { id: 'ANT_Deploy_Status', dp: 'Uint8', sizeB: 1, multiplier: false }, { id: 'GPS_Read_Time', dp: 'Uint32LE', sizeB: 4, multiplier: false }, { id: 'TOW', dp: 'FloatLE', sizeB: 4, multiplier: false }, { id: 'ECEF_X', dp: 'FloatLE', sizeB: 4, multiplier: false }, { id: 'ECEF_Y', dp: 'FloatLE', sizeB: 4, multiplier: false }, { id: 'ECEF_Z', dp: 'FloatLE', sizeB: 4, multiplier: false }, { id: 'ECEF_VX', dp: 'FloatLE', sizeB: 4, multiplier: false }, { id: 'ECEF_VY', dp: 'FloatLE', sizeB: 4, multiplier: false }, { id: 'ECEF_VZ', dp: 'FloatLE', sizeB: 4, multiplier: false }, { id: 'Seconds_From_2000', dp: 'FloatLE', sizeB: 4, multiplier: false }, { id: 'Gyro_1', dp: 'FloatLE', sizeB: 4, multiplier: false }, { id: 'Gyro_2', dp: 'FloatLE', sizeB: 4, multiplier: false }, { id: 'Gyro_3', dp: 'FloatLE', sizeB: 4, multiplier: false }, { id: 'Orbit_Propagator_Time', dp: 'int32LE', sizeB: 4, multiplier: false }, { id: 'Dof_9_Temperature', dp: 'FloatLE', sizeB: 4, multiplier: false }, { id: 'ECI_Z_1', dp: 'DoubleLE', sizeB: 8, multiplier: false }, { id: 'ECI_Z_2', dp: 'DoubleLE', sizeB: 8, multiplier: false }, { id: 'ECI_Z_3', dp: 'DoubleLE', sizeB: 8, multiplier: false }, { id: 'ECI_SUN_1', dp: 'DoubleLE', sizeB: 8, multiplier: false }, { id: 'ECI_SUN_2', dp: 'DoubleLE', sizeB: 8, multiplier: false }, { id: 'ECI_SUN_3', dp: 'DoubleLE', sizeB: 8, multiplier: false }, { id: 'ADCS_mode', dp: 'Uint8', sizeB: 1, multiplier: false }, { id: 'Reboot_Counter', dp: 'Uint32LE', sizeB: 4, multiplier: false }, { id: 'RSSI', dp: 'FloatBE', sizeB: 4, multiplier: false }, { id: 'GS_ID', dp: 'String', sizeB: 7, multiplier: false }]; 
var full_data = {}; 
var posFirst = 0; 
for (var i = 0; i < arr.length; i++) 
{ 
    var id = arr[i]['id']; var dp = arr[i]['dp']; 
var multiplier = arr[i]['multiplier'];
 var valu = ''; 
 var posSecond = posFirst + 2 * arr[i]['sizeB']; 
 if (dp == 'Uint8') { 
     valu = Buffer.from(HexData.slice(posFirst, posSecond), 'hex').readUInt8(); 
    } else if (dp === 'Uint16LE') { 
        valu = Buffer.from(HexData.slice(posFirst, posSecond), 'hex').readUInt16LE(); 
    } else if (dp === 'int32LE') { valu = Buffer.from(HexData.slice(posFirst, posSecond), 'hex').readInt32LE();
 } else if (dp === 'Uint32LE') { 
     valu = Buffer.from(HexData.slice(posFirst, posSecond), 'hex').readUInt32LE(); 
    } else if (dp === 'FloatLE') {
         valu = Buffer.from(HexData.slice(posFirst, posSecond), 'hex').readFloatLE();
         } else if (dp === 'FloatBE') { 
             valu = Buffer.from(HexData.slice(posFirst, posSecond), 'hex').readFloatBE(); 
            } else if (dp === 'DoubleLE') { 
                valu = Buffer.from(HexData.slice(posFirst, posSecond), 'hex').readDoubleLE();
             } else if (dp === 'String') { 
                 valu = Buffer.from(HexData.slice(posFirst, posSecond), 'hex').toString('utf8');
                 } else if (dp === 'hex') { valu = HexData.slice(posFirst, posSecond); } if (multiplier != false) { valu = valu * multiplier; } posFirst = posSecond; full_data[id] = valu; 


}; 
return full_data; };