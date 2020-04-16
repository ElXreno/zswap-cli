%global debug_package %{nil}

Name:           zswap-cli
Version:        1.0.1
Release:        1%{?dist}
Summary:        Utility for controlling zswap parameters

License:        ASL 2.0
URL:            https://github.com/ElXreno/zswap-cli
Source0:        %{url}/archive/v%{version}/%{name}-%{version}.tar.gz
Source1:        %{name}.conf
Source2:        %{name}.service

BuildRequires: cargo
BuildRequires: systemd

%description
Utility for controlling zswap parameters.


%prep
%autosetup


%build
cargo build --release


%install
install -m 0755 -Dp target/release/%{name} %{buildroot}%{_bindir}/%{name}
install -m 0644 -Dp %{SOURCE2} %{buildroot}%{_unitdir}/%{name}.service
install -m 0644 -Dp %{SOURCE1} %{buildroot}%{_sysconfdir}/%{name}.conf


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


%changelog
* Thu Apr 16 2020 ElXreno <elxreno@gmail.com> - 1.0.1-1
- Updated to version 1.0.1

* Sun Apr  5 2020 ElXreno <elxreno@gmail.com> - 1.0.0-1
- Initial packaging
