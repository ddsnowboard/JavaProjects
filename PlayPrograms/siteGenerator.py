#!/usr/bin/python3
with open("MasterCueList.txt", 'r') as f:
    with open("list.html", 'w') as w:
        tabs = lambda x: x.count("    ")
        depth = 0
        # w.write("""<html><head><style>*{font-family: monospace; } p { font-size: 1.4em;} .clickable{margin-top:0;margin-bottom:0}.clicked{background-color:#000000;color:#ffffff;}span{text-decoration:underline;}</style><script src="https://code.jquery.com/jquery-1.11.2.min.js"></script><script>$(document).ready(function(){var last=null;$(".clickable").click(function(){$(this).toggleClass("clicked");if(last!=null){last.toggleClass("clicked");} last=$(this);});$(document).keydown(function(event){if(event.which===37){last.toggleClass("clicked");$("#"+(parseInt(last.attr('id'),10)-1).toString()).toggleClass("clicked");last=$("#"+(parseInt(last.attr('id'),10)-1).toString());}else if(event.which===39){last.toggleClass("clicked");$("#"+(parseInt(last.attr('id'),10)+1).toString()).toggleClass("clicked");last=$("#"+(parseInt(last.attr('id'),10)+1).toString());} console.log("#"+(parseInt(last.attr('id'),10)+1).toString());});});</script><ul>""")
        w.write("""<html><head><style>*{font-family: monospace; } p { font-size: 1.4em;} .clickable{margin-top:0;margin-bottom:0}.clicked{background-color:#000000;color:#ffffff;}span{text-decoration:underline;}</style> <script> function toggleClass(el, cls) { if(el.classList.contains(cls)) el.classList.remove(cls); else el.classList.add(cls); } window.onload = function(){ var last = null; var clickables = document.getElementsByClassName("clickable"); for(var i = 0; i < clickables.length; i++) { clickables[i].addEventListener("click", function() { toggleClass(this, "clicked"); if(last && this !== last) last.classList.remove("clicked"); last = this; }); } document.onkeydown = function(e) { if(e.which === 37) { toggleClass(last, "clicked"); var el = document.getElementById(parseInt(last.getAttribute("id")) - 1); toggleClass(el, "clicked"); last = el; } else if(e.which === 39) { toggleClass(last, "clicked"); var el = document.getElementById(parseInt(last.getAttribute("id")) + 1); toggleClass(el, "clicked"); last = el; } }; }; </script>""")
        l = list(f)
        curr_line = 0
        for i in l:
            if tabs(i) > depth:
                w.write("<ul>")
            elif tabs(i) < depth:
                for p in range(depth - tabs(i)):
                    w.write("</ul></li>")
            w.write("<li><p id=\"{}\" class=\"clickable\">{}</p>".format(curr_line,i))
            curr_line += 1
            if tabs(i) < depth:
                for p in range(tabs(i)-depth):
                    w.write("</ul>")
            depth = tabs(i)


# Overture:
    # (1a) Opening BV: 
        # m3 BV:
# <li>Overture:
    # <ol>
        # <li>(1a) Opening BV: 
            # <ol>
                # <li>m3 BV: </li>
            # </ol>
        # </li>
    # </ol>
# </li>
