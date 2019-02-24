use crate::{Revise, NoPrev};

use std::fmt::{Write};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DistroIdV0 {
  Alpine,
  Centos,
  Debian,
  Fedora,
  RedHat,
  Ubuntu,
}

impl DistroIdV0 {
  pub fn to_desc_str(&self) -> &'static str {
    match self {
      &DistroIdV0::Alpine => "alpine",
      &DistroIdV0::Centos => "centos",
      &DistroIdV0::Debian => "debian",
      &DistroIdV0::Fedora => "fedora",
      &DistroIdV0::RedHat => "redhat",
      &DistroIdV0::Ubuntu => "ubuntu",
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DistroCodenameV0 {
  Alpine3_8,
  Alpine3_9,
  Centos6,
  Centos7,
  DebianWheezy,
  DebianJessie,
  DebianStretch,
  DebianBuster,
  UbuntuTrusty,
  UbuntuXenial,
  UbuntuBionic,
}

impl DistroCodenameV0 {
  pub fn to_id(&self) -> DistroIdV0 {
    match self {
      &DistroCodenameV0::Alpine3_8 |
      &DistroCodenameV0::Alpine3_9 => DistroIdV0::Alpine,
      &DistroCodenameV0::Centos6 |
      &DistroCodenameV0::Centos7 => DistroIdV0::Centos,
      &DistroCodenameV0::DebianWheezy |
      &DistroCodenameV0::DebianJessie |
      &DistroCodenameV0::DebianStretch |
      &DistroCodenameV0::DebianBuster => DistroIdV0::Debian,
      &DistroCodenameV0::UbuntuTrusty |
      &DistroCodenameV0::UbuntuXenial |
      &DistroCodenameV0::UbuntuBionic => DistroIdV0::Ubuntu,
    }
  }

  pub fn to_desc_str(&self) -> &'static str {
    match self {
      &DistroCodenameV0::Alpine3_8 => "alpine_3_8",
      &DistroCodenameV0::Alpine3_9 => "alpine_3_9",
      &DistroCodenameV0::Centos6 => "centos_6",
      &DistroCodenameV0::Centos7 => "centos_7",
      &DistroCodenameV0::DebianWheezy => "debian_wheezy",
      &DistroCodenameV0::DebianJessie => "debian_jessie",
      &DistroCodenameV0::DebianStretch => "debian_stretch",
      &DistroCodenameV0::DebianBuster => "debian_buster",
      &DistroCodenameV0::UbuntuTrusty => "ubuntu_trusty",
      &DistroCodenameV0::UbuntuXenial => "ubuntu_xenial",
      &DistroCodenameV0::UbuntuBionic => "ubuntu_bionic",
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct DistroInfoV0 {
  pub id: DistroIdV0,
  pub codename: Option<DistroCodenameV0>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct DriverVersionV0 {
  pub major: u32,
  pub minor: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CudaToolkitVersionV0 {
  Cuda6_5,
  Cuda7_0,
  Cuda7_5,
  Cuda8_0,
  Cuda9_0,
  Cuda9_1,
  Cuda9_2,
  Cuda10_0,
}

impl CudaToolkitVersionV0 {
  pub fn to_desc_str(&self) -> &'static str {
    match self {
      &CudaToolkitVersionV0::Cuda6_5 => "v6_5",
      &CudaToolkitVersionV0::Cuda7_0 => "v7_0",
      &CudaToolkitVersionV0::Cuda7_5 => "v7_5",
      &CudaToolkitVersionV0::Cuda8_0 => "v8_0",
      &CudaToolkitVersionV0::Cuda9_0 => "v9_0",
      &CudaToolkitVersionV0::Cuda9_1 => "v9_1",
      &CudaToolkitVersionV0::Cuda9_2 => "v9_2",
      &CudaToolkitVersionV0::Cuda10_0 => "v10_0",
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub struct PciSlotV0 {
  pub domain: Option<u32>,
  pub bus: u8,
  pub device: u8,
  pub function: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PciRecordV0 {
  pub slot: PciSlotV0,
  pub is_vga: bool,
  pub is_nvidia: bool,
  pub class: u16,
  pub vendor: u16,
  pub device: u16,
  //pub svendor: String,
  //pub sdevice: String,
  pub svendor: u16,
  pub sdevice: u16,
  pub rev: Option<u8>,
}

impl PciRecordV0 {
  pub fn is_gpu(&self) -> bool {
    self.is_vga && self.is_nvidia
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GpusV0 {
  pub pci_records: Vec<PciRecordV0>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemSetupV0 {
  pub distro_info: DistroInfoV0,
  pub driver_version: DriverVersionV0,
  pub num_cpus: usize,
  pub gpus: GpusV0,
}

impl SystemSetupV0 {
  pub fn prettyprinted(&self) -> String {
    let mut buf = String::new();
    writeln!(&mut buf, "distro: {}", self.distro_info.id.to_desc_str()).unwrap();
    if let Some(ref codename) = self.distro_info.codename {
      writeln!(&mut buf, "  codename: {}", codename.to_desc_str()).unwrap();
    }
    writeln!(&mut buf, "nvidia driver: {}.{}",
        self.driver_version.major, self.driver_version.minor).unwrap();
    writeln!(&mut buf, "num cpus: {}", self.num_cpus).unwrap();
    if !self.gpus.pci_records.is_empty() {
      writeln!(&mut buf, "gpus:").unwrap();
      for (idx, pci_record) in self.gpus.pci_records.iter().enumerate() {
        writeln!(&mut buf, "  gpu {}:", idx).unwrap();
        write!(&mut buf, "    pci slot: ").unwrap();
        if let Some(ref domain) = pci_record.slot.domain {
          write!(&mut buf, "{:08x}:", domain).unwrap();
        }
        writeln!(&mut buf, "{:02x}:{:02x}.{:02x}",
            pci_record.slot.bus,
            pci_record.slot.device,
            pci_record.slot.function,
        ).unwrap();
        write!(&mut buf, "    flags:").unwrap();
        if pci_record.is_vga {
          write!(&mut buf, " vga").unwrap();
        }
        if pci_record.is_nvidia {
          write!(&mut buf, " nvidia").unwrap();
        }
        writeln!(&mut buf, "").unwrap();
        writeln!(&mut buf, "    class: {:04x}",
            pci_record.class).unwrap();
        write!(&mut buf, "    vendor: {:04x} device: {:04x}",
            pci_record.vendor, pci_record.device).unwrap();
        if let Some(ref rev) = pci_record.rev {
          write!(&mut buf, " rev: {:02x}", rev).unwrap();
        }
        writeln!(&mut buf, "").unwrap();
        writeln!(&mut buf, "    sub vendor: {:04x} device: {:04x}",
            pci_record.svendor, pci_record.sdevice).unwrap();
      }
    }
    buf
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LocalDeviceV0 {
  PciSlot(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalMachineV0 {
  pub parallel_tasks: u32,
  pub gpus: Vec<LocalDeviceV0>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MachineConfigV0 {
  pub local_machine: LocalMachineV0,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserDomainV0 {
  GuppybotOrg,
  GithubCom,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserHandleV0 {
  pub username: String,
  pub domain: UserDomainV0,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CiEventPolicyV0 {
  Nobody,
  AllowedUsers,
  EverybodyExceptCiChanges,
  Everybody,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CiRepoV0 {
  pub remote_url: String,
  pub commit_policy: CiEventPolicyV0,
  pub pr_policy: CiEventPolicyV0,
  pub allowed_users: Vec<UserHandleV0>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CiConfigV0 {
  pub repos: Vec<CiRepoV0>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Bot2RegistryV0 {
  Auth{
    api_id: String,
  },
  Deauth{
    api_id: String,
  },
  RegisterCiMachine,
  RegisterCiRepo{
    repo_url: String,
  },
  RegisterMachine{
    api_id: String,
    root_manifest_id: String,
    system_setup: SystemSetupV0,
    machine_cfg: MachineConfigV0,
  },
  UnregisterCiMachine,
  UnregisterCiRepo,
  UnregisterMachine,
}

impl<'a> Revise<'a> for Bot2RegistryV0 {
  type RevisionPrev = NoPrev;

  fn revision() -> u32 {
    0
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Registry2BotV0 {
  Auth(Option<()>),
  Deauth(Option<()>),
  RegisterCiMachine(Option<()>),
  RegisterCiRepo(Option<()>),
  RegisterMachine(Option<()>),
  UnregisterCiMachine(Option<()>),
  UnregisterCiRepo(Option<()>),
  UnregisterMachine(Option<()>),
}

impl<'a> Revise<'a> for Registry2BotV0 {
  type RevisionPrev = NoPrev;

  fn revision() -> u32 {
    0
  }
}
