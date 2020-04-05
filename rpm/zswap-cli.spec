%global debug_package %{nil}

%global commit 9419b2f2f3e6c79de581d2b7dc5df83eed4de4b6

Name:           zswap-cli
Version:        0.1.0
Release:        1%{?dist}
Summary:        Utility for controlling zswap parameters

License:        ASL 2.0
URL:            https://github.com/ElXreno/zswap-cli
Source0:        %{url}/archive/%{commit}/%{name}-%{commit}.tar.gz
Source1:        %{name}.conf
Source2:        %{name}.service

BuildRequires: cargo
BuildRequires: systemd

%description
Utility for controlling zswap parameters


%prep
%autosetup -n %{name}-%{commit}


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
* Sun Apr  5 2020 ElXreno <elxreno@gmail.com>
- Initial packaging
