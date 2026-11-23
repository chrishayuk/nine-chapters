import json,urllib.request,urllib.parse,time,sys,os
UA="historical-reconstruction-research/0.1 (non-commercial)"
out=sys.argv[1]; f="File:Sibu Congkan0392-劉徽-九章算術-3-3.djvu"
for n in range(2,54):
    dst=os.path.join(out,f"p{n:02d}.jpg")
    if os.path.exists(dst) and os.path.getsize(dst)>10000: continue
    u="https://commons.wikimedia.org/w/api.php?action=query&format=json&prop=imageinfo&iiprop=url&iiurlwidth=1600&iiurlparam="+urllib.parse.quote(f"page{n}-1600px")+"&titles="+urllib.parse.quote(f)
    for attempt in range(4):
        try:
            j=json.loads(urllib.request.urlopen(urllib.request.Request(u,headers={"User-Agent":UA})).read())
            th=list(j["query"]["pages"].values())[0]["imageinfo"][0]["thumburl"]
            data=urllib.request.urlopen(urllib.request.Request(th,headers={"User-Agent":UA})).read()
            open(dst,"wb").write(data); print(n,len(data),flush=True); break
        except Exception as e:
            print(n,"retry",e,flush=True); time.sleep(20)
    time.sleep(5)
