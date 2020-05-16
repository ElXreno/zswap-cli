%global debug_package %{nil}

Name:           zswap-cli
Version:        1.0.3
Release:        2%{?dist}
Summary:        Utility for controlling zswap parameters

License:        MPLv2.0
URL:            https://github.com/ElXreno/zswap-cli
Source0:        %{url}/archive/v%{version}/%{name}-%{version}.tar.gz
Source1:        %{name}.conf
Source2:        %{name}.service
Source3:        %{name}.md

BuildRequires:  cargo
BuildRequires:  systemd
BuildRequires:  pandoc

%description
Utility for controlling zswap parameters.


%prep
%autosetup

mkdir ~/.cargo
echo '[source.crates-io]\nreplace-with = "vendored-sources"\n\n[source.vendored-sources]\ndirectory = "vendor"' > ~/.cargo/config


%build
cargo build --release --locked
strip target/release/%{name}
pandoc %{SOURCE3} -s -t man -o zswap-cli.1


%install
install -m 0755 -Dp target/release/%{name} %{buildroot}%{_bindir}/%{name}
install -m 0644 -Dp %{SOURCE2} %{buildroot}%{_unitdir}/%{name}.service
install -m 0644 -Dp %{SOURCE1} %{buildroot}%{_sysconfdir}/%{name}.conf

install -m 0644 -Dp %{name}.1 %{buildroot}%{_mandir}/man1/%{name}.1


%post
%systemd_post %{name}.service


%preun
%systemd_preun %{name}.service


%postun
%systemd_postun_with_restart %{name}.service


%files
%license LICENSE
%doc README.md
%{_bindir}/%{name}
%{_unitdir}/%{name}.service
%config(noreplace) %{_sysconfdir}/%{name}.conf
%{_mandir}/man1/%{name}.*


%changelog
* Sat May 16 2020 ElXreno <elxreno@gmail.com> - 1.0.3-2
- Bump spec

* Fri Apr 24 2020 ElXreno <elxreno@gmail.com> - 1.0.3-1
- Updated to version 1.0.3

* Fri Apr 17 2020 ElXreno <elxreno@gmail.com> - 1.0.2-1
- Updated to version 1.0.2

* Thu Apr 16 2020 ElXreno <elxreno@gmail.com> - 1.0.1-1
- Updated to version 1.0.1

* Sun Apr  5 2020 ElXreno <elxreno@gmail.com> - 1.0.0-1
- Initial packaging
