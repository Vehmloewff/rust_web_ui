use rust_web_ui::*;

pub struct Example38;

pub struct Example38Props {}

impl Default for Example38Props {
	fn default() -> Example38Props {
		Example38Props { }
	}
}

impl Widget<'_> for Example38 {
	type Props = Example38Props;

	fn render(mut ctx: Ctx<'_>, props: Example38Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("bg-white"), Style::PaddingY(96), Screen::Small(&[Style::PaddingY(128)])]);
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-7xl"), Style::PaddingX(24), Screen::Large(&[Style::PaddingX(32)])]);
				
				props.child("1", Dynamic).run("h2", |props| {
					props.styles(&[NoStyle::Noop("text-center"), NoStyle::Noop("text-lg"), Style::FontSemibold, NoStyle::Noop("leading-8"), Style::TextColor(Color::Fg(100))]);
					
					props.child("0", Label).run(|props| props.set_text("Trusted by the world’s most innovative teams"));
				});
				props.child("3", Dynamic).run("div", |props| {
					props.styles(&[NoStyle::Noop("mx-auto"), Style::MarginTop(40), NoStyle::Noop("grid"), NoStyle::Noop("max-w-lg"), NoStyle::Noop("grid-cols-4"), Style::ItemsCenter, NoStyle::Noop("gap-x-8"), NoStyle::Noop("gap-y-10"), Screen::Small(&[NoStyle::Noop("max-w-xl")]), Screen::Small(&[NoStyle::Noop("grid-cols-6")]), Screen::Small(&[NoStyle::Noop("gap-x-10")]), Screen::Large(&[Style::MarginX(0)]), Screen::Large(&[NoStyle::Noop("max-w-none")]), Screen::Large(&[NoStyle::Noop("grid-cols-5")])]);
					
					props.child("1", Dynamic).run("img", |props| {
						props.styles(&[NoStyle::Noop("col-span-2"), NoStyle::Noop("max-h-12"), NoStyle::Noop("w-full"), NoStyle::Noop("object-contain"), Screen::Large(&[NoStyle::Noop("col-span-1")])]);
						props.set_attribute("src", "https://tailwindui.com/img/logos/158x48/transistor-logo-gray-900.svg");
						props.set_attribute("alt", "Transistor");
						props.set_attribute("width", "158");
						props.set_attribute("height", "48");
					});
					props.child("3", Dynamic).run("img", |props| {
						props.styles(&[NoStyle::Noop("col-span-2"), NoStyle::Noop("max-h-12"), NoStyle::Noop("w-full"), NoStyle::Noop("object-contain"), Screen::Large(&[NoStyle::Noop("col-span-1")])]);
						props.set_attribute("src", "https://tailwindui.com/img/logos/158x48/reform-logo-gray-900.svg");
						props.set_attribute("alt", "Reform");
						props.set_attribute("width", "158");
						props.set_attribute("height", "48");
					});
					props.child("5", Dynamic).run("img", |props| {
						props.styles(&[NoStyle::Noop("col-span-2"), NoStyle::Noop("max-h-12"), NoStyle::Noop("w-full"), NoStyle::Noop("object-contain"), Screen::Large(&[NoStyle::Noop("col-span-1")])]);
						props.set_attribute("src", "https://tailwindui.com/img/logos/158x48/tuple-logo-gray-900.svg");
						props.set_attribute("alt", "Tuple");
						props.set_attribute("width", "158");
						props.set_attribute("height", "48");
					});
					props.child("7", Dynamic).run("img", |props| {
						props.styles(&[NoStyle::Noop("col-span-2"), NoStyle::Noop("max-h-12"), NoStyle::Noop("w-full"), NoStyle::Noop("object-contain"), Screen::Small(&[NoStyle::Noop("col-start-2")]), Screen::Large(&[NoStyle::Noop("col-span-1")])]);
						props.set_attribute("src", "https://tailwindui.com/img/logos/158x48/savvycal-logo-gray-900.svg");
						props.set_attribute("alt", "SavvyCal");
						props.set_attribute("width", "158");
						props.set_attribute("height", "48");
					});
					props.child("9", Dynamic).run("img", |props| {
						props.styles(&[NoStyle::Noop("col-span-2"), NoStyle::Noop("col-start-2"), NoStyle::Noop("max-h-12"), NoStyle::Noop("w-full"), NoStyle::Noop("object-contain"), Screen::Small(&[NoStyle::Noop("col-start-auto")]), Screen::Large(&[NoStyle::Noop("col-span-1")])]);
						props.set_attribute("src", "https://tailwindui.com/img/logos/158x48/statamic-logo-gray-900.svg");
						props.set_attribute("alt", "Statamic");
						props.set_attribute("width", "158");
						props.set_attribute("height", "48");
					});
				});
			});
		});
	}
}
