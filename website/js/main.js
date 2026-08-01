(function() {
    'use strict';

    // ==========================================
    // Dark/Light Theme
    // ==========================================
    var html = document.documentElement;
    var themeToggle = document.getElementById('theme-toggle');

    function getPreferredTheme() {
        return localStorage.getItem('kcm-theme') || 
            (window.matchMedia('(prefers-color-scheme: light)').matches ? 'light' : 'dark');
    }

    function setTheme(theme) {
        html.setAttribute('data-theme', theme);
        localStorage.setItem('kcm-theme', theme);
        if (themeToggle) {
            var sunIcon = themeToggle.querySelector('.theme-icon-sun');
            var moonIcon = themeToggle.querySelector('.theme-icon-moon');
            if (sunIcon && moonIcon) {
                sunIcon.style.display = theme === 'dark' ? 'inline-block' : 'none';
                moonIcon.style.display = theme === 'dark' ? 'none' : 'inline-block';
            }
        }
    }

    setTheme(getPreferredTheme());
    if (themeToggle) {
        themeToggle.addEventListener('click', function() {
            setTheme(html.getAttribute('data-theme') === 'dark' ? 'light' : 'dark');
        });
    }

    // ==========================================
    // Mobile Menu
    // ==========================================
    var menuBtn = document.querySelector('.mobile-menu-btn');
    var navLinks = document.querySelector('.nav-links');
    if (menuBtn && navLinks) {
        menuBtn.addEventListener('click', function() {
            var isOpen = navLinks.classList.toggle('open');
            menuBtn.setAttribute('aria-expanded', String(isOpen));
        });
        navLinks.querySelectorAll('a').forEach(function(link) {
            link.addEventListener('click', function() {
                navLinks.classList.remove('open');
                menuBtn.setAttribute('aria-expanded', 'false');
            });
        });
    }

    // ==========================================
    // Client-Side Search
    // ==========================================
    var searchInput = document.getElementById('doc-search');
    var searchResults = document.getElementById('search-results');
    if (searchInput && searchResults) {
        var docLinks = Array.from(document.querySelectorAll('.sidebar-nav a'));
        searchInput.addEventListener('input', function() {
            var query = this.value.toLowerCase().trim();
            searchResults.innerHTML = '';
            if (query.length < 2) { searchResults.style.display = 'none'; return; }
            var found = 0;
            docLinks.forEach(function(a) {
                var text = a.textContent.toLowerCase();
                var href = a.getAttribute('href') || '';
                if (text.includes(query) || href.toLowerCase().includes(query)) {
                    var li = document.createElement('li');
                    var link = a.cloneNode(true);
                    li.appendChild(link);
                    searchResults.appendChild(li);
                    found++;
                }
            });
            searchResults.style.display = found > 0 ? 'block' : 'none';
            if (found === 0) {
                var li = document.createElement('li');
                li.textContent = 'No results found';
                li.className = 'search-empty';
                searchResults.appendChild(li);
                searchResults.style.display = 'block';
            }
        });
        document.addEventListener('click', function(e) {
            if (!searchInput.contains(e.target) && !searchResults.contains(e.target)) {
                searchResults.style.display = 'none';
            }
        });
    }

    // ==========================================
    // Table of Contents Generation
    // ==========================================
    var tocContainer = document.getElementById('toc');
    var article = document.querySelector('.doc-article, article');
    if (tocContainer && article) {
        var headings = article.querySelectorAll('h2, h3');
        if (headings.length > 0) {
            var tocList = document.createElement('ul');
            tocList.className = 'toc-list';
            headings.forEach(function(h, i) {
                var id = h.id || 'heading-' + i;
                h.id = id;
                var li = document.createElement('li');
                li.className = h.tagName === 'H3' ? 'toc-h3' : 'toc-h2';
                var a = document.createElement('a');
                a.href = '#' + id;
                a.textContent = h.textContent;
                li.appendChild(a);
                tocList.appendChild(li);
            });
            tocContainer.appendChild(tocList);
        }
    }

    // ==========================================
    // Active Section Highlighting
    // ==========================================
    var navItems = document.querySelectorAll('.sidebar-nav a, .toc-list a');
    if (navItems.length > 0 && 'IntersectionObserver' in window) {
        var headings = Array.from(document.querySelectorAll('h2[id], h3[id]'));
        var headingMap = {};
        headings.forEach(function(h) {
            headingMap[h.id] = h;
        });
        
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

    // ==========================================
    // Code Copy Buttons
    // ==========================================
    document.querySelectorAll('pre').forEach(function(pre) {
        var btn = document.createElement('button');
        btn.className = 'copy-btn';
        btn.innerHTML = '<svg class="icon icon-sm" viewBox="0 0 24 24"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/></svg> Copy';
        btn.setAttribute('aria-label', 'Copy code to clipboard');
        btn.addEventListener('click', function() {
            var code = pre.querySelector('code');
            var text = code ? code.textContent : pre.textContent;
            navigator.clipboard.writeText(text).then(function() {
                btn.innerHTML = '<svg class="icon icon-sm" viewBox="0 0 24 24"><polyline points="20 6 9 17 4 12"/></svg> Copied!';
                btn.classList.add('copied');
                setTimeout(function() {
                    btn.innerHTML = '<svg class="icon icon-sm" viewBox="0 0 24 24"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/></svg> Copy';
                    btn.classList.remove('copied');
                }, 2000);
            });
        });
        pre.style.position = 'relative';
        pre.appendChild(btn);
    });

    // ==========================================
    // Smooth Scroll
    // ==========================================
    document.querySelectorAll('a[href^="#"]').forEach(function(anchor) {
        anchor.addEventListener('click', function(e) {
            var target = document.querySelector(this.getAttribute('href'));
            if (target) { e.preventDefault(); target.scrollIntoView({ behavior: 'smooth', block: 'start' }); }
        });
    });

    // ==========================================
    // Keyboard Navigation
    // ==========================================
    document.addEventListener('keydown', function(e) {
        if (e.key === 'Escape' && navLinks && navLinks.classList.contains('open')) {
            navLinks.classList.remove('open');
            menuBtn.setAttribute('aria-expanded', 'false');
        }
        if (e.key === '/' && document.activeElement.tagName !== 'INPUT') {
            e.preventDefault();
            var searchEl = document.getElementById('doc-search');
            if (searchEl) searchEl.focus();
        }
    });

    // ==========================================
    // Scroll Animations
    // ==========================================
    if ('IntersectionObserver' in window) {
        var animObserver = new IntersectionObserver(function(entries) {
            entries.forEach(function(entry) {
                if (entry.isIntersecting) entry.target.classList.add('visible');
            });
        }, { threshold: 0.1 });
        document.querySelectorAll('.feature-card, .doc-card, .crate-card, .metric').forEach(function(el) {
            el.classList.add('fade-in');
            animObserver.observe(el);
        });
    }

    // ==========================================
    // Version Badge
    // ==========================================
    var badges = document.querySelectorAll('.version-badge');
    badges.forEach(function(badge) {
        var stored = localStorage.getItem('kcm-version');
        if (stored) badge.textContent = stored;
    });

})();
