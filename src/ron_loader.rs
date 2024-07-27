/// asset loader macro
#[allow(dead_code, unused)]
#[macro_export]
macro_rules! ron_asset_loader {
    (
        $plugin_name : ident,
        $asset_loader_name : ident,
        $asset_type : ident,
        $extensions : expr,
        $( $field_name:ident -> $handle_name:ident ),*
        $(; $list_field:ident -> ( $sub_field:ident -> $sub_handle:ident ))*
        $(= $array_name:ident -> $array_handle:ident)*
        $(? $vec_assets:ident -> $vec_handles:ident)*
    ) => {
        use bevy::{
            asset::{AssetLoader,AsyncReadExt},
            prelude::Plugin,
        };

        pub struct $plugin_name;
        impl Plugin for $plugin_name {
            fn build(&self, app: &mut App) {
                app.init_asset::<$asset_type>();
                app.register_asset_loader($asset_loader_name);
            }
        }

        pub struct $asset_loader_name;
        impl AssetLoader for $asset_loader_name {

            type Asset = $asset_type;
            type Settings = ();
            type Error = bevy::asset::LoadDirectError;

            fn load<'a>(
                &'a self,
                reader: &'a mut bevy::asset::io::Reader,
                _settings: &'a Self::Settings,
                load_context: &'a mut bevy::asset::LoadContext,
            ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
                Box::pin(async move {
                    let mut bytes = Vec::new();
                    reader.read_to_end(&mut bytes).await.unwrap();
                    let mut asset = ron::de::from_bytes::<$asset_type>(bytes.as_slice()).unwrap();

                    $(
                        asset.$handle_name = load_context.load(&asset.$field_name);
                    )*

                    $(
                        for sub_asset in &mut asset.$list_field {
                            sub_asset.$sub_handle = load_context.load(&sub_asset.$sub_field);
                        }
                    )*

                    $(
                        for sub_asset in asset.$array_name.iter() {
                            asset.$array_handle.push(load_context.load(sub_asset));
                        }
                    )*


                    $(
                        for sub_asset in asset.$vec_assets.iter() {
                            let handle = load_context.add_labeled_asset("".to_string(), sub_asset.clone());
                            asset.$vec_handles.push(handle);
                        }
                    )*

                    Ok(asset)
                })
            }
            fn extensions(&self) -> &[&str] {
                $extensions
            }
        }
    };
}
