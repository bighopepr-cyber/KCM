/* KCM Engineering Portal — JavaScript v3.0 */
(function() {
    'use strict';

    // ---- Theme ----
    var html = document.documentElement;
    var btn = document.getElementById('theme-toggle');
    function setTheme(t) { html.setAttribute('data-theme', t); try { localStorage.setItem('kcm-theme', t); } catch(e) {} }
    var s; try { s = localStorage.getItem('kcm-theme'); } catch(e) {}
    setTheme(s || (window.matchMedia('(prefers-color-scheme: light)').matches ? 'light' : 'dark'));
    if (btn) btn.addEventListener('click', function() { setTheme(html.getAttribute('data-theme') === 'dark' ? 'light' : 'dark'); });

    // ---- Mobile Menu ----
    var mb = document.querySelector('.mobile-menu-btn');
    var nl = document.querySelector('.nav-links');
    if (mb && nl) {
        mb.addEventListener('click', function() {
            var o = nl.classList.toggle('open');
            mb.setAttribute('aria-expanded', String(o));
        });
        nl.querySelectorAll('a').forEach(function(a) { a.addEventListener('click', function() { nl.classList.remove('open'); mb.setAttribute('aria-expanded', 'false'); }); });
    }

    // ---- Smooth Scroll ----
    document.querySelectorAll('a[href^="#"]').forEach(function(a) {
        a.addEventListener('click', function(e) {
            var el = document.querySelector(this.getAttribute('href'));
            if (el) { e.preventDefault(); el.scrollIntoView({ behavior: 'smooth', block: 'start' }); }
        });
    });

    // ---- Table of Contents Generation ----
    var toc = document.getElementById('toc');
    var article = document.querySelector('.doc-article, article');
    if (toc && article) {
        var headings = article.querySelectorAll('h2, h3');
        if (headings.length > 0) {
            var ul = document.createElement('ul');
            ul.className = 'toc-list';
            headings.forEach(function(h, i) {
                var id = h.id || 'section-' + i;
                h.id = id;
                var li = document.createElement('li');
                li.className = h.tagName === 'H3' ? 'toc-h3' : 'toc-h2';
                var a = document.createElement('a');
                a.href = '#' + id;
                a.textContent = h.textContent;
                li.appendChild(a);
                ul.appendChild(li);
            });
            toc.appendChild(ul);
        }
    }

    // ---- Active Section Highlighting ----
    var navItems = document.querySelectorAll('.sidebar-nav a, .toc-list a');
    if (navItems.length > 0 && 'IntersectionObserver' in window) {
        var headings = Array.from(document.querySelectorAll('h2[id], h3[id]'));
        var observer = new IntersectionObserver(function(entries) {
            entries.forEach(function(entry) {
                if (entry.isIntersecting) {
                    var id = entry.target.id;
                    navItems.forEach(function(a) {
                        a.classList.toggle('active', a.getAttribute('href') === '#' + id);
                    });
                }
            });
        }, { rootMargin: '-80px 0px -60% 0px' });
        headings.forEach(function(h) { observer.observe(h); });
    }

    // ---- Copy to Clipboard ----
    document.querySelectorAll('pre').forEach(function(pre) {
        var btn = document.createElement('button');
        btn.className = 'copy-btn';
        btn.textContent = 'Copy';
        btn.setAttribute('aria-label', 'Copy code');
        btn.addEventListener('click', function() {
            var code = pre.querySelector('code');
            var text = code ? code.textContent : pre.textContent;
            navigator.clipboard.writeText(text).then(function() {
                btn.textContent = 'Copied';
                setTimeout(function() { btn.textContent = 'Copy'; }, 1500);
            })['catch'](function() {});
        });
        pre.style.position = 'relative';
        pre.appendChild(btn);
    });

    // ---- Client-Side Search (docs pages) ----
    var searchInput = document.getElementById('doc-search');
    var searchResults = document.getElementById('search-results');
    if (searchInput && searchResults) {
        var docLinks = Array.from(document.querySelectorAll('.sidebar-nav a'));
        searchInput.addEventListener('input', function() {
            var q = this.value.toLowerCase().trim();
            searchResults.innerHTML = '';
            if (q.length < 2) { searchResults.style.display = 'none'; return; }
            var n = 0;
            docLinks.forEach(function(a) {
                var t = a.textContent.toLowerCase();
                if (t.indexOf(q) !== -1) {
                    var li = document.createElement('li');
                    li.appendChild(a.cloneNode(true));
                    searchResults.appendChild(li);
                    n++;
                }
            });
            if (n === 0) { var li = document.createElement('li'); li.className = 'search-empty'; li.textContent = 'No results found'; searchResults.appendChild(li); }
            searchResults.style.display = 'block';
        });
        document.addEventListener('click', function(e) {
            if (!searchInput.contains(e.target) && !searchResults.contains(e.target)) searchResults.style.display = 'none';
        });
    }

    // ---- Heading Anchor Links ----
    document.querySelectorAll('.doc-article h2[id], .doc-article h3[id]').forEach(function(h) {
        var a = document.createElement('a');
        a.className = 'heading-anchor';
        a.href = '#' + h.id;
        a.textContent = '#';
        a.setAttribute('aria-hidden', 'true');
        h.appendChild(a);
    });

    // ---- Scroll Animations ----
    if ('IntersectionObserver' in window) {
        var obs = new IntersectionObserver(function(entries) {
            entries.forEach(function(e) { if (e.isIntersecting) e.target.classList.add('visible'); });
        }, { threshold: 0.1 });
        document.querySelectorAll('.card, .stat-card').forEach(function(el) { el.classList.add('fade-in'); obs.observe(el); });
    }

    // ---- Keyboard Shortcuts ----
    document.addEventListener('keydown', function(e) {
        if (e.key === 'Escape' && nl && nl.classList.contains('open')) {
            nl.classList.remove('open');
            mb.setAttribute('aria-expanded', 'false');
        }
    });
})();
