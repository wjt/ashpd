use std::os::unix::prelude::AsRawFd;

use ashpd::{
    desktop::print::{PageSetup, PrintProxy, Settings},
    zbus, WindowIdentifier,
};
use gtk::glib::{self, clone};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp {
    use adw::subclass::prelude::*;
    use gtk::CompositeTemplate;

    use super::*;

    #[derive(Debug, CompositeTemplate, Default)]
    #[template(resource = "/com/belmoussaoui/ashpd/demo/print.ui")]
    pub struct PrintPage {
        #[template_child]
        pub title: TemplateChild<gtk::Entry>,
        #[template_child]
        pub modal_switch: TemplateChild<gtk::Switch>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PrintPage {
        const NAME: &'static str = "PrintPage";
        type Type = super::PrintPage;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
            klass.set_layout_manager_type::<adw::ClampLayout>();
            klass.install_action("print.select_file", None, move |page, _action, _target| {
                let ctx = glib::MainContext::default();
                ctx.spawn_local(clone!(@weak page => async move {
                    page.select_file().await;
                }));
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }
    impl ObjectImpl for PrintPage {}
    impl WidgetImpl for PrintPage {}
    impl BinImpl for PrintPage {}
}

glib::wrapper! {
    pub struct PrintPage(ObjectSubclass<imp::PrintPage>) @extends gtk::Widget, adw::Bin;
}

impl PrintPage {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create a PrintPage")
    }

    async fn select_file(&self) {
        let self_ = imp::PrintPage::from_instance(self);
        let title = self_.title.text();
        let modal = self_.modal_switch.is_active();
        let root = self.native().unwrap();
        let identifier = WindowIdentifier::from_native(&root).await;

        let file_chooser = gtk::FileChooserNativeBuilder::new()
            .accept_label("Select")
            .action(gtk::FileChooserAction::Open)
            .modal(true)
            .transient_for(root.downcast_ref::<gtk::Window>().unwrap())
            .build();
        let filter = gtk::FileFilter::new();
        filter.add_pixbuf_formats();
        filter.set_name(Some("images"));
        file_chooser.add_filter(&filter);

        if file_chooser.run_future().await == gtk::ResponseType::Accept {
            let path = file_chooser.file().unwrap().path().unwrap();
            let file = std::fs::File::open(path).unwrap();

            if let Err(err) = print(&identifier, &title, file, modal).await {
                tracing::error!("Failed to print {}", err);
            }
        };

        file_chooser.destroy();
    }
}

async fn print<F: AsRawFd>(
    identifier: &WindowIdentifier,
    title: &str,
    file: F,
    modal: bool,
) -> ashpd::Result<()> {
    let cnx = zbus::azync::Connection::session().await?;
    let proxy = PrintProxy::new(&cnx).await?;

    let out = proxy
        .prepare_print(
            identifier,
            title,
            Settings::default(),
            PageSetup::default(),
            modal,
        )
        .await?;

    proxy
        .print(identifier, title, &file, Some(out.token), modal)
        .await?;

    Ok(())
}