#!/usr/bin/env python3
import sys
import xml.etree.ElementTree as ET

def format_byte_string(input_string):
    output_list = []

    # 2文字ずつ分割してリストに格納
    split_strings = [input_string[i:i+2] for i in range(0, len(input_string), 2)]

    # 最後の要素が1文字の場合、0を追加
    if len(split_strings[-1]) == 1:
        split_strings[-1] += '0'

    # リストの要素を16進数形式で整形して出力リストに追加
    for byte in split_strings:
        output_list.append(f"0x{byte}")

    # 出力リストを指定された形式で結合して返す
    formatted_output = "&[" + ", ".join(output_list) + "]"
    return formatted_output

def process_xml(xml_file):
    # XMLファイルをパース
    tree = ET.parse(xml_file)
    root = tree.getroot()


    # item要素を抽出
    for item in root.findall('remocon'):
        name = item.find('header/remoconname').text.replace("/", "|")
        print(f"\t\"{name}\" => phf_map! {{")
    
        for button in item.findall('signal/button'):
            button_name = button.find('buttonname').text.replace("/", "|")
            code = button.find('code').text
            print(f"\t\t\"{button_name}\" => {format_byte_string(code)},")
        print("\t},")

# コマンドライン引数からXMLファイルのパスを取得
if len(sys.argv) < 2:
    print("Usage: python script.py <xml_file1> <xml_file2> ...")
    sys.exit(1)

print("// auto generated file from export xml")
print("use phf::phf_map;")
print("")
print("pub static DEVICES: phf::Map<&'static str, phf::Map<&'static str, &'static [u8]>> = phf_map! {")

for xml_file in sys.argv[1:]:
    process_xml(xml_file)

print("};")


