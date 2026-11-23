import json,urllib.request,urllib.parse,time,re,html,sys,os
UA="historical-reconstruction-research/0.1 (non-commercial)"
out=sys.argv[1]
for p in ["序","卷第一","卷第二","卷第三","卷第四","卷第五","卷第六","卷第七","卷第八","卷第九","音義一卷"]:
    t="九章算術 (四部叢刊本)/"+p
    u="https://zh.wikisource.org/w/api.php?action=parse&format=json&prop=text&page="+urllib.parse.quote(t)
    r=urllib.request.urlopen(urllib.request.Request(u,headers={"User-Agent":UA})).read()
    h=json.loads(r)["parse"]["text"]["*"]
    open(os.path.join(out,p+".html"),"w").write(h)
    txt=re.sub(r"<(br|p|div)[^>]*>","\n",h); txt=re.sub(r"<[^>]+>","",txt); txt=html.unescape(txt)
    txt=re.sub(r"\n\s*\n+","\n\n",txt)
    open(os.path.join(out,p+".txt"),"w").write(txt)
    print(p,len(txt)); time.sleep(4)
