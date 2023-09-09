# 変数定義
DIR_XML := ../resource/xml
DIR_GEN_RS := ../src/rs_wfirex4_api

# フォルダ内のすべての.xmlファイルのリストを取得
XML_FILES := $(wildcard $(DIR_XML)/*.xml)

# .jsonファイルを .rsファイルに変換するためのターゲットを生成
RS_FILE := $(DIR_GEN_RS)/devices.rs

#変換用pythonscript
CONV_SCRIPT := xml_preload.py

# Makefile内での前処理スクリプトの呼び出し
.PHONY: all
all: $(RS_FILE)

$(DIR_GEN_RS):
	mkdir -p $(DIR_GEN_RS)


$(RS_FILE): $(DIR_GEN_RS) $(XML_FILES) $(CONV_SCRIPT) 
	python3 $(CONV_SCRIPT) $(XML_FILES) >$@

clean:
	rm -f $(RS_FILE)
