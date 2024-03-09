--[[ youRC-Tools
Copyright (C) 2024  Chris H. Meyer

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
--]]

--[[ https://www.usb.org/sites/default/files/midi10.pdf ]]



midi_protocol = Proto("Midi",  "USB Midi Protocol")

local sysex_midi = ProtoField.bytes("midi.sysex_midi", "SysEx", base.SPACE)
midi_protocol.fields = { sysex_midi }

function parse_sysex(buffer)
  local len = buffer:len()
  local bytes = ByteArray.new()
  
  for i=0, len-1, 4
  do 
    local cin = bit.band(buffer(i,1):uint(), 0xf)
    
    if cin == 0x4 then
      bytes = bytes .. buffer(i+1, 3):bytes()
    elseif cin == 0x7 then
      bytes = bytes .. buffer(i+1, 3):bytes()
      return bytes
    elseif cin == 0x6 then
      bytes = bytes .. buffer(i+1, 2):bytes()
      return bytes
    elseif cin == 0x5  then
      bytes = bytes .. buffer(i+1, 1):bytes()
      return bytes
    end
  end
  
  return bytes
end

function midi_protocol.dissector(buffer, pinfo, tree)
  local len = buffer:len()
  if length == 0 then return end

  pinfo.cols.protocol = midi_protocol.name
  
  local frame_start = -1
  local frame_len = 0

  local subtree = tree:add(midi_protocol, buffer(), "USB MIDI Data")

  for i = 0, len-1, 4
  do
    local cin = bit.band(buffer(i,1):uint(), 0xf)
    if cin == 0x4
    then
      if frame_start == -1 then
        frame_start = i
      end

      frame_len = frame_len+4
    elseif cin == 0x7 or cin == 0x6 or cin == 0x5 then
      local usb_midi_sysex = buffer(frame_start, frame_len+4)
      local sysex = parse_sysex(usb_midi_sysex)
      item = subtree:add(sysex_midi, usb_midi_sysex);
      item:set_text("SysEx: " .. sysex:tohex(true, " "))
      frame_len = 0
      frame_start = -1
    end
  end

end

local usb_table = DissectorTable.get("usb.bulk")
usb_table:add(0xffff, midi_protocol)