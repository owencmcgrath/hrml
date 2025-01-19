export const DEFAULT_CONTENT = `jf hey
jff welcome to the HRML demo
jfff you can do up to six headings!
This is some bold js text sj, some italic jd text dj and some underlined ju text uj!
kl let's create a list!
ja info
ja more info
jl first
jl second
jl third!
kll some code!
jkd python
# Some Python!
print("this is so much better than Markdown, right? ;)")
dkj
jffff cool car!
jh [dream car, really] gh [https://i.kinja-img.com/image/upload/c_fill,h_675,pg_1,q_80,w_1200/823337c1eb4bc7e0ffc884d3eaf1fb22.jpg] hj
js
jg [check out my other work!] gh [https://owencmcgrath.com] hg`;

export const DEBOUNCE_DELAY = 250;

export const PDF_OPTIONS = {
    margin: 1,
    filename: 'hrml-document.pdf',
    image: { type: 'jpeg', quality: 0.98 },
    html2canvas: { scale: 2 },
    jsPDF: { unit: 'in', format: 'letter', orientation: 'portrait' }
};
