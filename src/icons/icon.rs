#[derive(Clone, Copy, Debug)]
pub enum Icon {
    Info,
    Ok,
    X,
    Exclamation,
    Question,
    BellBounce,
    Bug,
    ExternalLink,
    Bell,
    Heart,
    Donation,
    Star,
    Email,
    DownloadCloud,
    UploadCloud,
    DonwloadSquare,
    UploadSquare,
    Key,
    Lock,
    MagnifyingGlass,
    FolderMagnifyingGlass,
    Target,
    FileMagnifyingGlass,
    Discord,
    Youtube,
    Instagram,
    Twitter,
    Tiktok,
    SkullBones,
    Blood,
    Trash,
}

pub trait IconTrait {
    fn to_vec() -> Vec<Icon>;
}

impl IconTrait for Icon {
    fn to_vec() -> Vec<Icon> {
        use Icon::*;
        vec![
            Info,
            Ok,
            X,
            Exclamation,
            Question,
            BellBounce,
            Bug,
            ExternalLink,
            Bell,
            Heart,
            Star,
            Donation,
            Email,
            DownloadCloud,
            UploadCloud,
            DonwloadSquare,
            UploadSquare,
            Key,
            Lock,
            MagnifyingGlass,
            FolderMagnifyingGlass,
            Target,
            FileMagnifyingGlass,
            Discord,
            Youtube,
            Instagram,
            Twitter,
            Tiktok,
            SkullBones,
            Blood,
            Trash,
        ]
    }
}
