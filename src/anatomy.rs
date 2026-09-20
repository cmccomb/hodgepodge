//! The conventional adult skeleton and a selected set of skeletal muscles.
#![allow(missing_docs)]

dataset_enum! {
    /// Anatomical left and right refer to the person, not the viewer.
    pub enum BodySide {
        Left => "Left",
        Right => "Right",
        Midline => "Midline",
    }
}

dataset_enum! {
    /// The two principal divisions of the adult skeleton.
    pub enum SkeletalDivision {
        Axial => "Axial",
        Appendicular => "Appendicular",
    }
}

dataset_enum! {
    /// Regions used to group individual bones in the 206-bone teaching convention.
    pub enum BoneRegion {
        Skull => "Skull",
        Ear => "Ear",
        Neck => "Neck",
        VertebralColumn => "Vertebral Column",
        ThoracicCage => "Thoracic Cage",
        PectoralGirdle => "Pectoral Girdle",
        UpperLimb => "Upper Limb",
        Hand => "Hand",
        PelvicGirdle => "Pelvic Girdle",
        LowerLimb => "Lower Limb",
        Foot => "Foot",
    }
}

impl BoneRegion {
    /// Returns the skeletal division containing this region.
    #[must_use]
    pub const fn division(self) -> SkeletalDivision {
        match self {
            Self::Skull | Self::Ear | Self::Neck | Self::VertebralColumn | Self::ThoracicCage => {
                SkeletalDivision::Axial
            }
            Self::PectoralGirdle
            | Self::UpperLimb
            | Self::Hand
            | Self::PelvicGirdle
            | Self::LowerLimb
            | Self::Foot => SkeletalDivision::Appendicular,
        }
    }
}

dataset_enum! {
    /// The conventional 206 adult bones, with paired bones represented individually.
    /// Anatomical variation and accessory sesamoids are excluded. Teeth are not bones.
    /// Sacrum, coccyx, sternum and each hip bone are counted as fused adult bones.
    /// Digits are numbered 1–5 from thumb or great toe; neither has a middle phalanx.
    /// Cervical vertebrae 1 and 2 are the atlas and axis.
    pub enum Bone {
        Frontal => "Frontal",
        Occipital => "Occipital",
        Sphenoid => "Sphenoid",
        Ethmoid => "Ethmoid",
        Vomer => "Vomer",
        Mandible => "Mandible",
        LeftParietal => "Left Parietal",
        RightParietal => "Right Parietal",
        LeftTemporal => "Left Temporal",
        RightTemporal => "Right Temporal",
        LeftNasal => "Left Nasal",
        RightNasal => "Right Nasal",
        LeftMaxilla => "Left Maxilla",
        RightMaxilla => "Right Maxilla",
        LeftZygomatic => "Left Zygomatic",
        RightZygomatic => "Right Zygomatic",
        LeftLacrimal => "Left Lacrimal",
        RightLacrimal => "Right Lacrimal",
        LeftPalatine => "Left Palatine",
        RightPalatine => "Right Palatine",
        LeftInferiorNasalConcha => "Left Inferior Nasal Concha",
        RightInferiorNasalConcha => "Right Inferior Nasal Concha",
        LeftMalleus => "Left Malleus",
        RightMalleus => "Right Malleus",
        LeftIncus => "Left Incus",
        RightIncus => "Right Incus",
        LeftStapes => "Left Stapes",
        RightStapes => "Right Stapes",
        Hyoid => "Hyoid",
        CervicalVertebra1 => "Cervical Vertebra 1",
        CervicalVertebra2 => "Cervical Vertebra 2",
        CervicalVertebra3 => "Cervical Vertebra 3",
        CervicalVertebra4 => "Cervical Vertebra 4",
        CervicalVertebra5 => "Cervical Vertebra 5",
        CervicalVertebra6 => "Cervical Vertebra 6",
        CervicalVertebra7 => "Cervical Vertebra 7",
        ThoracicVertebra1 => "Thoracic Vertebra 1",
        ThoracicVertebra2 => "Thoracic Vertebra 2",
        ThoracicVertebra3 => "Thoracic Vertebra 3",
        ThoracicVertebra4 => "Thoracic Vertebra 4",
        ThoracicVertebra5 => "Thoracic Vertebra 5",
        ThoracicVertebra6 => "Thoracic Vertebra 6",
        ThoracicVertebra7 => "Thoracic Vertebra 7",
        ThoracicVertebra8 => "Thoracic Vertebra 8",
        ThoracicVertebra9 => "Thoracic Vertebra 9",
        ThoracicVertebra10 => "Thoracic Vertebra 10",
        ThoracicVertebra11 => "Thoracic Vertebra 11",
        ThoracicVertebra12 => "Thoracic Vertebra 12",
        LumbarVertebra1 => "Lumbar Vertebra 1",
        LumbarVertebra2 => "Lumbar Vertebra 2",
        LumbarVertebra3 => "Lumbar Vertebra 3",
        LumbarVertebra4 => "Lumbar Vertebra 4",
        LumbarVertebra5 => "Lumbar Vertebra 5",
        Sacrum => "Sacrum",
        Coccyx => "Coccyx",
        Sternum => "Sternum",
        LeftRib1 => "Left Rib 1",
        RightRib1 => "Right Rib 1",
        LeftRib2 => "Left Rib 2",
        RightRib2 => "Right Rib 2",
        LeftRib3 => "Left Rib 3",
        RightRib3 => "Right Rib 3",
        LeftRib4 => "Left Rib 4",
        RightRib4 => "Right Rib 4",
        LeftRib5 => "Left Rib 5",
        RightRib5 => "Right Rib 5",
        LeftRib6 => "Left Rib 6",
        RightRib6 => "Right Rib 6",
        LeftRib7 => "Left Rib 7",
        RightRib7 => "Right Rib 7",
        LeftRib8 => "Left Rib 8",
        RightRib8 => "Right Rib 8",
        LeftRib9 => "Left Rib 9",
        RightRib9 => "Right Rib 9",
        LeftRib10 => "Left Rib 10",
        RightRib10 => "Right Rib 10",
        LeftRib11 => "Left Rib 11",
        RightRib11 => "Right Rib 11",
        LeftRib12 => "Left Rib 12",
        RightRib12 => "Right Rib 12",
        LeftClavicle => "Left Clavicle",
        RightClavicle => "Right Clavicle",
        LeftScapula => "Left Scapula",
        RightScapula => "Right Scapula",
        LeftHumerus => "Left Humerus",
        RightHumerus => "Right Humerus",
        LeftRadius => "Left Radius",
        RightRadius => "Right Radius",
        LeftUlna => "Left Ulna",
        RightUlna => "Right Ulna",
        LeftScaphoid => "Left Scaphoid",
        RightScaphoid => "Right Scaphoid",
        LeftLunate => "Left Lunate",
        RightLunate => "Right Lunate",
        LeftTriquetrum => "Left Triquetrum",
        RightTriquetrum => "Right Triquetrum",
        LeftPisiform => "Left Pisiform",
        RightPisiform => "Right Pisiform",
        LeftTrapezium => "Left Trapezium",
        RightTrapezium => "Right Trapezium",
        LeftTrapezoid => "Left Trapezoid",
        RightTrapezoid => "Right Trapezoid",
        LeftCapitate => "Left Capitate",
        RightCapitate => "Right Capitate",
        LeftHamate => "Left Hamate",
        RightHamate => "Right Hamate",
        LeftMetacarpal1 => "Left Metacarpal 1",
        RightMetacarpal1 => "Right Metacarpal 1",
        LeftMetacarpal2 => "Left Metacarpal 2",
        RightMetacarpal2 => "Right Metacarpal 2",
        LeftMetacarpal3 => "Left Metacarpal 3",
        RightMetacarpal3 => "Right Metacarpal 3",
        LeftMetacarpal4 => "Left Metacarpal 4",
        RightMetacarpal4 => "Right Metacarpal 4",
        LeftMetacarpal5 => "Left Metacarpal 5",
        RightMetacarpal5 => "Right Metacarpal 5",
        LeftHandProximalPhalanx1 => "Left Hand Proximal Phalanx 1",
        RightHandProximalPhalanx1 => "Right Hand Proximal Phalanx 1",
        LeftHandProximalPhalanx2 => "Left Hand Proximal Phalanx 2",
        RightHandProximalPhalanx2 => "Right Hand Proximal Phalanx 2",
        LeftHandProximalPhalanx3 => "Left Hand Proximal Phalanx 3",
        RightHandProximalPhalanx3 => "Right Hand Proximal Phalanx 3",
        LeftHandProximalPhalanx4 => "Left Hand Proximal Phalanx 4",
        RightHandProximalPhalanx4 => "Right Hand Proximal Phalanx 4",
        LeftHandProximalPhalanx5 => "Left Hand Proximal Phalanx 5",
        RightHandProximalPhalanx5 => "Right Hand Proximal Phalanx 5",
        LeftHandMiddlePhalanx2 => "Left Hand Middle Phalanx 2",
        RightHandMiddlePhalanx2 => "Right Hand Middle Phalanx 2",
        LeftHandMiddlePhalanx3 => "Left Hand Middle Phalanx 3",
        RightHandMiddlePhalanx3 => "Right Hand Middle Phalanx 3",
        LeftHandMiddlePhalanx4 => "Left Hand Middle Phalanx 4",
        RightHandMiddlePhalanx4 => "Right Hand Middle Phalanx 4",
        LeftHandMiddlePhalanx5 => "Left Hand Middle Phalanx 5",
        RightHandMiddlePhalanx5 => "Right Hand Middle Phalanx 5",
        LeftHandDistalPhalanx1 => "Left Hand Distal Phalanx 1",
        RightHandDistalPhalanx1 => "Right Hand Distal Phalanx 1",
        LeftHandDistalPhalanx2 => "Left Hand Distal Phalanx 2",
        RightHandDistalPhalanx2 => "Right Hand Distal Phalanx 2",
        LeftHandDistalPhalanx3 => "Left Hand Distal Phalanx 3",
        RightHandDistalPhalanx3 => "Right Hand Distal Phalanx 3",
        LeftHandDistalPhalanx4 => "Left Hand Distal Phalanx 4",
        RightHandDistalPhalanx4 => "Right Hand Distal Phalanx 4",
        LeftHandDistalPhalanx5 => "Left Hand Distal Phalanx 5",
        RightHandDistalPhalanx5 => "Right Hand Distal Phalanx 5",
        LeftHipBone => "Left Hip Bone",
        RightHipBone => "Right Hip Bone",
        LeftFemur => "Left Femur",
        RightFemur => "Right Femur",
        LeftPatella => "Left Patella",
        RightPatella => "Right Patella",
        LeftTibia => "Left Tibia",
        RightTibia => "Right Tibia",
        LeftFibula => "Left Fibula",
        RightFibula => "Right Fibula",
        LeftTalus => "Left Talus",
        RightTalus => "Right Talus",
        LeftCalcaneus => "Left Calcaneus",
        RightCalcaneus => "Right Calcaneus",
        LeftNavicular => "Left Navicular",
        RightNavicular => "Right Navicular",
        LeftCuboid => "Left Cuboid",
        RightCuboid => "Right Cuboid",
        LeftMedialCuneiform => "Left Medial Cuneiform",
        RightMedialCuneiform => "Right Medial Cuneiform",
        LeftIntermediateCuneiform => "Left Intermediate Cuneiform",
        RightIntermediateCuneiform => "Right Intermediate Cuneiform",
        LeftLateralCuneiform => "Left Lateral Cuneiform",
        RightLateralCuneiform => "Right Lateral Cuneiform",
        LeftMetatarsal1 => "Left Metatarsal 1",
        RightMetatarsal1 => "Right Metatarsal 1",
        LeftMetatarsal2 => "Left Metatarsal 2",
        RightMetatarsal2 => "Right Metatarsal 2",
        LeftMetatarsal3 => "Left Metatarsal 3",
        RightMetatarsal3 => "Right Metatarsal 3",
        LeftMetatarsal4 => "Left Metatarsal 4",
        RightMetatarsal4 => "Right Metatarsal 4",
        LeftMetatarsal5 => "Left Metatarsal 5",
        RightMetatarsal5 => "Right Metatarsal 5",
        LeftFootProximalPhalanx1 => "Left Foot Proximal Phalanx 1",
        RightFootProximalPhalanx1 => "Right Foot Proximal Phalanx 1",
        LeftFootProximalPhalanx2 => "Left Foot Proximal Phalanx 2",
        RightFootProximalPhalanx2 => "Right Foot Proximal Phalanx 2",
        LeftFootProximalPhalanx3 => "Left Foot Proximal Phalanx 3",
        RightFootProximalPhalanx3 => "Right Foot Proximal Phalanx 3",
        LeftFootProximalPhalanx4 => "Left Foot Proximal Phalanx 4",
        RightFootProximalPhalanx4 => "Right Foot Proximal Phalanx 4",
        LeftFootProximalPhalanx5 => "Left Foot Proximal Phalanx 5",
        RightFootProximalPhalanx5 => "Right Foot Proximal Phalanx 5",
        LeftFootMiddlePhalanx2 => "Left Foot Middle Phalanx 2",
        RightFootMiddlePhalanx2 => "Right Foot Middle Phalanx 2",
        LeftFootMiddlePhalanx3 => "Left Foot Middle Phalanx 3",
        RightFootMiddlePhalanx3 => "Right Foot Middle Phalanx 3",
        LeftFootMiddlePhalanx4 => "Left Foot Middle Phalanx 4",
        RightFootMiddlePhalanx4 => "Right Foot Middle Phalanx 4",
        LeftFootMiddlePhalanx5 => "Left Foot Middle Phalanx 5",
        RightFootMiddlePhalanx5 => "Right Foot Middle Phalanx 5",
        LeftFootDistalPhalanx1 => "Left Foot Distal Phalanx 1",
        RightFootDistalPhalanx1 => "Right Foot Distal Phalanx 1",
        LeftFootDistalPhalanx2 => "Left Foot Distal Phalanx 2",
        RightFootDistalPhalanx2 => "Right Foot Distal Phalanx 2",
        LeftFootDistalPhalanx3 => "Left Foot Distal Phalanx 3",
        RightFootDistalPhalanx3 => "Right Foot Distal Phalanx 3",
        LeftFootDistalPhalanx4 => "Left Foot Distal Phalanx 4",
        RightFootDistalPhalanx4 => "Right Foot Distal Phalanx 4",
        LeftFootDistalPhalanx5 => "Left Foot Distal Phalanx 5",
        RightFootDistalPhalanx5 => "Right Foot Distal Phalanx 5",
    }
}

impl Bone {
    /// Returns the anatomical region.
    #[must_use]
    #[allow(clippy::too_many_lines)] // Exhaustive metadata for 206 individual bones.
    pub const fn region(self) -> BoneRegion {
        match self {
            Self::Frontal
            | Self::Occipital
            | Self::Sphenoid
            | Self::Ethmoid
            | Self::Vomer
            | Self::Mandible
            | Self::LeftParietal
            | Self::RightParietal
            | Self::LeftTemporal
            | Self::RightTemporal
            | Self::LeftNasal
            | Self::RightNasal
            | Self::LeftMaxilla
            | Self::RightMaxilla
            | Self::LeftZygomatic
            | Self::RightZygomatic
            | Self::LeftLacrimal
            | Self::RightLacrimal
            | Self::LeftPalatine
            | Self::RightPalatine
            | Self::LeftInferiorNasalConcha
            | Self::RightInferiorNasalConcha => BoneRegion::Skull,
            Self::LeftMalleus
            | Self::RightMalleus
            | Self::LeftIncus
            | Self::RightIncus
            | Self::LeftStapes
            | Self::RightStapes => BoneRegion::Ear,
            Self::Hyoid => BoneRegion::Neck,
            Self::CervicalVertebra1
            | Self::CervicalVertebra2
            | Self::CervicalVertebra3
            | Self::CervicalVertebra4
            | Self::CervicalVertebra5
            | Self::CervicalVertebra6
            | Self::CervicalVertebra7
            | Self::ThoracicVertebra1
            | Self::ThoracicVertebra2
            | Self::ThoracicVertebra3
            | Self::ThoracicVertebra4
            | Self::ThoracicVertebra5
            | Self::ThoracicVertebra6
            | Self::ThoracicVertebra7
            | Self::ThoracicVertebra8
            | Self::ThoracicVertebra9
            | Self::ThoracicVertebra10
            | Self::ThoracicVertebra11
            | Self::ThoracicVertebra12
            | Self::LumbarVertebra1
            | Self::LumbarVertebra2
            | Self::LumbarVertebra3
            | Self::LumbarVertebra4
            | Self::LumbarVertebra5
            | Self::Sacrum
            | Self::Coccyx => BoneRegion::VertebralColumn,
            Self::Sternum
            | Self::LeftRib1
            | Self::RightRib1
            | Self::LeftRib2
            | Self::RightRib2
            | Self::LeftRib3
            | Self::RightRib3
            | Self::LeftRib4
            | Self::RightRib4
            | Self::LeftRib5
            | Self::RightRib5
            | Self::LeftRib6
            | Self::RightRib6
            | Self::LeftRib7
            | Self::RightRib7
            | Self::LeftRib8
            | Self::RightRib8
            | Self::LeftRib9
            | Self::RightRib9
            | Self::LeftRib10
            | Self::RightRib10
            | Self::LeftRib11
            | Self::RightRib11
            | Self::LeftRib12
            | Self::RightRib12 => BoneRegion::ThoracicCage,
            Self::LeftClavicle | Self::RightClavicle | Self::LeftScapula | Self::RightScapula => {
                BoneRegion::PectoralGirdle
            }
            Self::LeftHumerus
            | Self::RightHumerus
            | Self::LeftRadius
            | Self::RightRadius
            | Self::LeftUlna
            | Self::RightUlna => BoneRegion::UpperLimb,
            Self::LeftScaphoid
            | Self::RightScaphoid
            | Self::LeftLunate
            | Self::RightLunate
            | Self::LeftTriquetrum
            | Self::RightTriquetrum
            | Self::LeftPisiform
            | Self::RightPisiform
            | Self::LeftTrapezium
            | Self::RightTrapezium
            | Self::LeftTrapezoid
            | Self::RightTrapezoid
            | Self::LeftCapitate
            | Self::RightCapitate
            | Self::LeftHamate
            | Self::RightHamate
            | Self::LeftMetacarpal1
            | Self::RightMetacarpal1
            | Self::LeftMetacarpal2
            | Self::RightMetacarpal2
            | Self::LeftMetacarpal3
            | Self::RightMetacarpal3
            | Self::LeftMetacarpal4
            | Self::RightMetacarpal4
            | Self::LeftMetacarpal5
            | Self::RightMetacarpal5
            | Self::LeftHandProximalPhalanx1
            | Self::RightHandProximalPhalanx1
            | Self::LeftHandProximalPhalanx2
            | Self::RightHandProximalPhalanx2
            | Self::LeftHandProximalPhalanx3
            | Self::RightHandProximalPhalanx3
            | Self::LeftHandProximalPhalanx4
            | Self::RightHandProximalPhalanx4
            | Self::LeftHandProximalPhalanx5
            | Self::RightHandProximalPhalanx5
            | Self::LeftHandMiddlePhalanx2
            | Self::RightHandMiddlePhalanx2
            | Self::LeftHandMiddlePhalanx3
            | Self::RightHandMiddlePhalanx3
            | Self::LeftHandMiddlePhalanx4
            | Self::RightHandMiddlePhalanx4
            | Self::LeftHandMiddlePhalanx5
            | Self::RightHandMiddlePhalanx5
            | Self::LeftHandDistalPhalanx1
            | Self::RightHandDistalPhalanx1
            | Self::LeftHandDistalPhalanx2
            | Self::RightHandDistalPhalanx2
            | Self::LeftHandDistalPhalanx3
            | Self::RightHandDistalPhalanx3
            | Self::LeftHandDistalPhalanx4
            | Self::RightHandDistalPhalanx4
            | Self::LeftHandDistalPhalanx5
            | Self::RightHandDistalPhalanx5 => BoneRegion::Hand,
            Self::LeftHipBone | Self::RightHipBone => BoneRegion::PelvicGirdle,
            Self::LeftFemur
            | Self::RightFemur
            | Self::LeftPatella
            | Self::RightPatella
            | Self::LeftTibia
            | Self::RightTibia
            | Self::LeftFibula
            | Self::RightFibula => BoneRegion::LowerLimb,
            Self::LeftTalus
            | Self::RightTalus
            | Self::LeftCalcaneus
            | Self::RightCalcaneus
            | Self::LeftNavicular
            | Self::RightNavicular
            | Self::LeftCuboid
            | Self::RightCuboid
            | Self::LeftMedialCuneiform
            | Self::RightMedialCuneiform
            | Self::LeftIntermediateCuneiform
            | Self::RightIntermediateCuneiform
            | Self::LeftLateralCuneiform
            | Self::RightLateralCuneiform
            | Self::LeftMetatarsal1
            | Self::RightMetatarsal1
            | Self::LeftMetatarsal2
            | Self::RightMetatarsal2
            | Self::LeftMetatarsal3
            | Self::RightMetatarsal3
            | Self::LeftMetatarsal4
            | Self::RightMetatarsal4
            | Self::LeftMetatarsal5
            | Self::RightMetatarsal5
            | Self::LeftFootProximalPhalanx1
            | Self::RightFootProximalPhalanx1
            | Self::LeftFootProximalPhalanx2
            | Self::RightFootProximalPhalanx2
            | Self::LeftFootProximalPhalanx3
            | Self::RightFootProximalPhalanx3
            | Self::LeftFootProximalPhalanx4
            | Self::RightFootProximalPhalanx4
            | Self::LeftFootProximalPhalanx5
            | Self::RightFootProximalPhalanx5
            | Self::LeftFootMiddlePhalanx2
            | Self::RightFootMiddlePhalanx2
            | Self::LeftFootMiddlePhalanx3
            | Self::RightFootMiddlePhalanx3
            | Self::LeftFootMiddlePhalanx4
            | Self::RightFootMiddlePhalanx4
            | Self::LeftFootMiddlePhalanx5
            | Self::RightFootMiddlePhalanx5
            | Self::LeftFootDistalPhalanx1
            | Self::RightFootDistalPhalanx1
            | Self::LeftFootDistalPhalanx2
            | Self::RightFootDistalPhalanx2
            | Self::LeftFootDistalPhalanx3
            | Self::RightFootDistalPhalanx3
            | Self::LeftFootDistalPhalanx4
            | Self::RightFootDistalPhalanx4
            | Self::LeftFootDistalPhalanx5
            | Self::RightFootDistalPhalanx5 => BoneRegion::Foot,
        }
    }
}

impl Bone {
    /// Returns the anatomical side or midline.
    #[must_use]
    #[allow(clippy::too_many_lines)] // Exhaustive metadata for 206 individual bones.
    pub const fn side(self) -> BodySide {
        match self {
            Self::Frontal
            | Self::Occipital
            | Self::Sphenoid
            | Self::Ethmoid
            | Self::Vomer
            | Self::Mandible
            | Self::Hyoid
            | Self::CervicalVertebra1
            | Self::CervicalVertebra2
            | Self::CervicalVertebra3
            | Self::CervicalVertebra4
            | Self::CervicalVertebra5
            | Self::CervicalVertebra6
            | Self::CervicalVertebra7
            | Self::ThoracicVertebra1
            | Self::ThoracicVertebra2
            | Self::ThoracicVertebra3
            | Self::ThoracicVertebra4
            | Self::ThoracicVertebra5
            | Self::ThoracicVertebra6
            | Self::ThoracicVertebra7
            | Self::ThoracicVertebra8
            | Self::ThoracicVertebra9
            | Self::ThoracicVertebra10
            | Self::ThoracicVertebra11
            | Self::ThoracicVertebra12
            | Self::LumbarVertebra1
            | Self::LumbarVertebra2
            | Self::LumbarVertebra3
            | Self::LumbarVertebra4
            | Self::LumbarVertebra5
            | Self::Sacrum
            | Self::Coccyx
            | Self::Sternum => BodySide::Midline,
            Self::LeftParietal
            | Self::LeftTemporal
            | Self::LeftNasal
            | Self::LeftMaxilla
            | Self::LeftZygomatic
            | Self::LeftLacrimal
            | Self::LeftPalatine
            | Self::LeftInferiorNasalConcha
            | Self::LeftMalleus
            | Self::LeftIncus
            | Self::LeftStapes
            | Self::LeftRib1
            | Self::LeftRib2
            | Self::LeftRib3
            | Self::LeftRib4
            | Self::LeftRib5
            | Self::LeftRib6
            | Self::LeftRib7
            | Self::LeftRib8
            | Self::LeftRib9
            | Self::LeftRib10
            | Self::LeftRib11
            | Self::LeftRib12
            | Self::LeftClavicle
            | Self::LeftScapula
            | Self::LeftHumerus
            | Self::LeftRadius
            | Self::LeftUlna
            | Self::LeftScaphoid
            | Self::LeftLunate
            | Self::LeftTriquetrum
            | Self::LeftPisiform
            | Self::LeftTrapezium
            | Self::LeftTrapezoid
            | Self::LeftCapitate
            | Self::LeftHamate
            | Self::LeftMetacarpal1
            | Self::LeftMetacarpal2
            | Self::LeftMetacarpal3
            | Self::LeftMetacarpal4
            | Self::LeftMetacarpal5
            | Self::LeftHandProximalPhalanx1
            | Self::LeftHandProximalPhalanx2
            | Self::LeftHandProximalPhalanx3
            | Self::LeftHandProximalPhalanx4
            | Self::LeftHandProximalPhalanx5
            | Self::LeftHandMiddlePhalanx2
            | Self::LeftHandMiddlePhalanx3
            | Self::LeftHandMiddlePhalanx4
            | Self::LeftHandMiddlePhalanx5
            | Self::LeftHandDistalPhalanx1
            | Self::LeftHandDistalPhalanx2
            | Self::LeftHandDistalPhalanx3
            | Self::LeftHandDistalPhalanx4
            | Self::LeftHandDistalPhalanx5
            | Self::LeftHipBone
            | Self::LeftFemur
            | Self::LeftPatella
            | Self::LeftTibia
            | Self::LeftFibula
            | Self::LeftTalus
            | Self::LeftCalcaneus
            | Self::LeftNavicular
            | Self::LeftCuboid
            | Self::LeftMedialCuneiform
            | Self::LeftIntermediateCuneiform
            | Self::LeftLateralCuneiform
            | Self::LeftMetatarsal1
            | Self::LeftMetatarsal2
            | Self::LeftMetatarsal3
            | Self::LeftMetatarsal4
            | Self::LeftMetatarsal5
            | Self::LeftFootProximalPhalanx1
            | Self::LeftFootProximalPhalanx2
            | Self::LeftFootProximalPhalanx3
            | Self::LeftFootProximalPhalanx4
            | Self::LeftFootProximalPhalanx5
            | Self::LeftFootMiddlePhalanx2
            | Self::LeftFootMiddlePhalanx3
            | Self::LeftFootMiddlePhalanx4
            | Self::LeftFootMiddlePhalanx5
            | Self::LeftFootDistalPhalanx1
            | Self::LeftFootDistalPhalanx2
            | Self::LeftFootDistalPhalanx3
            | Self::LeftFootDistalPhalanx4
            | Self::LeftFootDistalPhalanx5 => BodySide::Left,
            Self::RightParietal
            | Self::RightTemporal
            | Self::RightNasal
            | Self::RightMaxilla
            | Self::RightZygomatic
            | Self::RightLacrimal
            | Self::RightPalatine
            | Self::RightInferiorNasalConcha
            | Self::RightMalleus
            | Self::RightIncus
            | Self::RightStapes
            | Self::RightRib1
            | Self::RightRib2
            | Self::RightRib3
            | Self::RightRib4
            | Self::RightRib5
            | Self::RightRib6
            | Self::RightRib7
            | Self::RightRib8
            | Self::RightRib9
            | Self::RightRib10
            | Self::RightRib11
            | Self::RightRib12
            | Self::RightClavicle
            | Self::RightScapula
            | Self::RightHumerus
            | Self::RightRadius
            | Self::RightUlna
            | Self::RightScaphoid
            | Self::RightLunate
            | Self::RightTriquetrum
            | Self::RightPisiform
            | Self::RightTrapezium
            | Self::RightTrapezoid
            | Self::RightCapitate
            | Self::RightHamate
            | Self::RightMetacarpal1
            | Self::RightMetacarpal2
            | Self::RightMetacarpal3
            | Self::RightMetacarpal4
            | Self::RightMetacarpal5
            | Self::RightHandProximalPhalanx1
            | Self::RightHandProximalPhalanx2
            | Self::RightHandProximalPhalanx3
            | Self::RightHandProximalPhalanx4
            | Self::RightHandProximalPhalanx5
            | Self::RightHandMiddlePhalanx2
            | Self::RightHandMiddlePhalanx3
            | Self::RightHandMiddlePhalanx4
            | Self::RightHandMiddlePhalanx5
            | Self::RightHandDistalPhalanx1
            | Self::RightHandDistalPhalanx2
            | Self::RightHandDistalPhalanx3
            | Self::RightHandDistalPhalanx4
            | Self::RightHandDistalPhalanx5
            | Self::RightHipBone
            | Self::RightFemur
            | Self::RightPatella
            | Self::RightTibia
            | Self::RightFibula
            | Self::RightTalus
            | Self::RightCalcaneus
            | Self::RightNavicular
            | Self::RightCuboid
            | Self::RightMedialCuneiform
            | Self::RightIntermediateCuneiform
            | Self::RightLateralCuneiform
            | Self::RightMetatarsal1
            | Self::RightMetatarsal2
            | Self::RightMetatarsal3
            | Self::RightMetatarsal4
            | Self::RightMetatarsal5
            | Self::RightFootProximalPhalanx1
            | Self::RightFootProximalPhalanx2
            | Self::RightFootProximalPhalanx3
            | Self::RightFootProximalPhalanx4
            | Self::RightFootProximalPhalanx5
            | Self::RightFootMiddlePhalanx2
            | Self::RightFootMiddlePhalanx3
            | Self::RightFootMiddlePhalanx4
            | Self::RightFootMiddlePhalanx5
            | Self::RightFootDistalPhalanx1
            | Self::RightFootDistalPhalanx2
            | Self::RightFootDistalPhalanx3
            | Self::RightFootDistalPhalanx4
            | Self::RightFootDistalPhalanx5 => BodySide::Right,
        }
    }
}

impl Bone {
    /// Returns the axial or appendicular division.
    #[must_use]
    pub const fn division(self) -> SkeletalDivision {
        self.region().division()
    }
}

dataset_enum! {
    /// Broad muscle locations; muscles crossing joints are grouped for teaching.
    pub enum MuscleRegion {
        HeadAndNeck => "Head and Neck",
        Trunk => "Trunk",
        UpperLimb => "Upper Limb",
        LowerLimb => "Lower Limb",
    }
}

dataset_enum! {
    /// A teaching selection of named human skeletal muscles, not a complete inventory.
    /// Names represent muscle types without separate left/right instances.
    /// Locations are broad teaching groups, not origin, insertion, or action claims.
    pub enum Muscle {
        Occipitofrontalis => "Occipitofrontalis",
        OrbicularisOculi => "Orbicularis Oculi",
        OrbicularisOris => "Orbicularis Oris",
        Buccinator => "Buccinator",
        ZygomaticusMajor => "Zygomaticus Major",
        ZygomaticusMinor => "Zygomaticus Minor",
        Masseter => "Masseter",
        Temporalis => "Temporalis",
        MedialPterygoid => "Medial Pterygoid",
        LateralPterygoid => "Lateral Pterygoid",
        Platysma => "Platysma",
        Sternocleidomastoid => "Sternocleidomastoid",
        SpleniusCapitis => "Splenius Capitis",
        Trapezius => "Trapezius",
        LatissimusDorsi => "Latissimus Dorsi",
        RhomboidMajor => "Rhomboid Major",
        RhomboidMinor => "Rhomboid Minor",
        LevatorScapulae => "Levator Scapulae",
        SerratusAnterior => "Serratus Anterior",
        PectoralisMajor => "Pectoralis Major",
        PectoralisMinor => "Pectoralis Minor",
        Subclavius => "Subclavius",
        Diaphragm => "Diaphragm",
        RectusAbdominis => "Rectus Abdominis",
        ExternalOblique => "External Oblique",
        InternalOblique => "Internal Oblique",
        TransversusAbdominis => "Transversus Abdominis",
        Deltoid => "Deltoid",
        Supraspinatus => "Supraspinatus",
        Infraspinatus => "Infraspinatus",
        TeresMinor => "Teres Minor",
        Subscapularis => "Subscapularis",
        TeresMajor => "Teres Major",
        Coracobrachialis => "Coracobrachialis",
        BicepsBrachii => "Biceps Brachii",
        Brachialis => "Brachialis",
        TricepsBrachii => "Triceps Brachii",
        Anconeus => "Anconeus",
        Brachioradialis => "Brachioradialis",
        PronatorTeres => "Pronator Teres",
        PronatorQuadratus => "Pronator Quadratus",
        Supinator => "Supinator",
        FlexorCarpiRadialis => "Flexor Carpi Radialis",
        FlexorCarpiUlnaris => "Flexor Carpi Ulnaris",
        PalmarisLongus => "Palmaris Longus",
        FlexorDigitorumSuperficialis => "Flexor Digitorum Superficialis",
        FlexorDigitorumProfundus => "Flexor Digitorum Profundus",
        FlexorPollicisLongus => "Flexor Pollicis Longus",
        ExtensorCarpiRadialisLongus => "Extensor Carpi Radialis Longus",
        ExtensorCarpiRadialisBrevis => "Extensor Carpi Radialis Brevis",
        ExtensorCarpiUlnaris => "Extensor Carpi Ulnaris",
        ExtensorDigitorum => "Extensor Digitorum",
        ExtensorPollicisLongus => "Extensor Pollicis Longus",
        ExtensorPollicisBrevis => "Extensor Pollicis Brevis",
        AbductorPollicisLongus => "Abductor Pollicis Longus",
        Iliacus => "Iliacus",
        PsoasMajor => "Psoas Major",
        GluteusMaximus => "Gluteus Maximus",
        GluteusMedius => "Gluteus Medius",
        GluteusMinimus => "Gluteus Minimus",
        TensorFasciaeLatae => "Tensor Fasciae Latae",
        Sartorius => "Sartorius",
        RectusFemoris => "Rectus Femoris",
        VastusLateralis => "Vastus Lateralis",
        VastusMedialis => "Vastus Medialis",
        VastusIntermedius => "Vastus Intermedius",
        Pectineus => "Pectineus",
        AdductorLongus => "Adductor Longus",
        AdductorBrevis => "Adductor Brevis",
        AdductorMagnus => "Adductor Magnus",
        Gracilis => "Gracilis",
        BicepsFemoris => "Biceps Femoris",
        Semitendinosus => "Semitendinosus",
        Semimembranosus => "Semimembranosus",
        Piriformis => "Piriformis",
        ObturatorInternus => "Obturator Internus",
        ObturatorExternus => "Obturator Externus",
        GemellusSuperior => "Gemellus Superior",
        GemellusInferior => "Gemellus Inferior",
        QuadratusFemoris => "Quadratus Femoris",
        TibialisAnterior => "Tibialis Anterior",
        ExtensorDigitorumLongus => "Extensor Digitorum Longus",
        ExtensorHallucisLongus => "Extensor Hallucis Longus",
        FibularisLongus => "Fibularis Longus",
        FibularisBrevis => "Fibularis Brevis",
        Gastrocnemius => "Gastrocnemius",
        Soleus => "Soleus",
        Plantaris => "Plantaris",
        Popliteus => "Popliteus",
        TibialisPosterior => "Tibialis Posterior",
        FlexorDigitorumLongus => "Flexor Digitorum Longus",
        FlexorHallucisLongus => "Flexor Hallucis Longus",
    }
}

impl Muscle {
    /// Returns the broad teaching location.
    #[must_use]
    pub const fn region(self) -> MuscleRegion {
        match self {
            Self::Occipitofrontalis
            | Self::OrbicularisOculi
            | Self::OrbicularisOris
            | Self::Buccinator
            | Self::ZygomaticusMajor
            | Self::ZygomaticusMinor
            | Self::Masseter
            | Self::Temporalis
            | Self::MedialPterygoid
            | Self::LateralPterygoid
            | Self::Platysma
            | Self::Sternocleidomastoid
            | Self::SpleniusCapitis => MuscleRegion::HeadAndNeck,
            Self::Trapezius
            | Self::LatissimusDorsi
            | Self::RhomboidMajor
            | Self::RhomboidMinor
            | Self::LevatorScapulae
            | Self::SerratusAnterior
            | Self::PectoralisMajor
            | Self::PectoralisMinor
            | Self::Subclavius
            | Self::Diaphragm
            | Self::RectusAbdominis
            | Self::ExternalOblique
            | Self::InternalOblique
            | Self::TransversusAbdominis => MuscleRegion::Trunk,
            Self::Deltoid
            | Self::Supraspinatus
            | Self::Infraspinatus
            | Self::TeresMinor
            | Self::Subscapularis
            | Self::TeresMajor
            | Self::Coracobrachialis
            | Self::BicepsBrachii
            | Self::Brachialis
            | Self::TricepsBrachii
            | Self::Anconeus
            | Self::Brachioradialis
            | Self::PronatorTeres
            | Self::PronatorQuadratus
            | Self::Supinator
            | Self::FlexorCarpiRadialis
            | Self::FlexorCarpiUlnaris
            | Self::PalmarisLongus
            | Self::FlexorDigitorumSuperficialis
            | Self::FlexorDigitorumProfundus
            | Self::FlexorPollicisLongus
            | Self::ExtensorCarpiRadialisLongus
            | Self::ExtensorCarpiRadialisBrevis
            | Self::ExtensorCarpiUlnaris
            | Self::ExtensorDigitorum
            | Self::ExtensorPollicisLongus
            | Self::ExtensorPollicisBrevis
            | Self::AbductorPollicisLongus => MuscleRegion::UpperLimb,
            Self::Iliacus
            | Self::PsoasMajor
            | Self::GluteusMaximus
            | Self::GluteusMedius
            | Self::GluteusMinimus
            | Self::TensorFasciaeLatae
            | Self::Sartorius
            | Self::RectusFemoris
            | Self::VastusLateralis
            | Self::VastusMedialis
            | Self::VastusIntermedius
            | Self::Pectineus
            | Self::AdductorLongus
            | Self::AdductorBrevis
            | Self::AdductorMagnus
            | Self::Gracilis
            | Self::BicepsFemoris
            | Self::Semitendinosus
            | Self::Semimembranosus
            | Self::Piriformis
            | Self::ObturatorInternus
            | Self::ObturatorExternus
            | Self::GemellusSuperior
            | Self::GemellusInferior
            | Self::QuadratusFemoris
            | Self::TibialisAnterior
            | Self::ExtensorDigitorumLongus
            | Self::ExtensorHallucisLongus
            | Self::FibularisLongus
            | Self::FibularisBrevis
            | Self::Gastrocnemius
            | Self::Soleus
            | Self::Plantaris
            | Self::Popliteus
            | Self::TibialisPosterior
            | Self::FlexorDigitorumLongus
            | Self::FlexorHallucisLongus => MuscleRegion::LowerLimb,
        }
    }
}
