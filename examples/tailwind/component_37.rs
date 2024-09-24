use rust_web_ui::*;

pub struct Example37;

pub struct Example37Props {}

impl Default for Example37Props {
	fn default() -> Example37Props {
		Example37Props { }
	}
}

impl Widget<'_> for Example37 {
	type Props = Example37Props;

	fn render(mut ctx: Ctx<'_>, _props: Example37Props) {
		ctx.styles(vec![Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(vec![Style::Noop("bg-white"), Style::PaddingY(Size::Exact(96)), Style::OnScreen(Screen::Small, &[Style::PaddingY(Size::Exact(128))])]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(vec![Style::Noop("mx-auto"), Style::Noop("max-w-7xl"), Style::PaddingX(Size::Exact(24)), Style::OnScreen(Screen::Large, &[Style::PaddingX(Size::Exact(32))])]);
				
				props.child("1", Dynamic).run("h2", |mut props| {
					props.styles(vec![Style::Noop("text-center"), Style::Noop("text-lg"), Style::FontSemibold, Style::Noop("leading-8"), Style::TextColor(Color::Fg(100))]);
					
					props.child("0", Label).run(|props| props.text("Trusted by the world’s most innovative teams"));
				});
				props.child("3", Dynamic).run("div", |mut props| {
					props.styles(vec![Style::Noop("mx-auto"), Style::MarginTop(Size::Exact(40)), Style::Noop("grid"), Style::Noop("max-w-lg"), Style::Noop("grid-cols-4"), Style::ItemsCenter, Style::Noop("gap-x-8"), Style::Noop("gap-y-10"), Style::OnScreen(Screen::Small, &[Style::Noop("max-w-xl")]), Style::OnScreen(Screen::Small, &[Style::Noop("grid-cols-6")]), Style::OnScreen(Screen::Small, &[Style::Noop("gap-x-10")]), Style::OnScreen(Screen::Large, &[Style::MarginX(Size::Exact(0))]), Style::OnScreen(Screen::Large, &[Style::Noop("max-w-none")]), Style::OnScreen(Screen::Large, &[Style::Noop("grid-cols-5")])]);
					
					props.child("1", Dynamic).run("img", |mut props| {
						props.styles(vec![Style::Noop("col-span-2"), Style::Noop("max-h-12"), Style::Width(Size::Full), Style::Noop("object-contain"), Style::OnScreen(Screen::Large, &[Style::Noop("col-span-1")])]);
						props.set_attribute("src", "https://tailwindui.com/img/logos/158x48/transistor-logo-gray-900.svg");
						props.set_attribute("alt", "Transistor");
						props.set_attribute("width", "158");
						props.set_attribute("height", "48");
					});
					props.child("3", Dynamic).run("img", |mut props| {
						props.styles(vec![Style::Noop("col-span-2"), Style::Noop("max-h-12"), Style::Width(Size::Full), Style::Noop("object-contain"), Style::OnScreen(Screen::Large, &[Style::Noop("col-span-1")])]);
						props.set_attribute("src", "https://tailwindui.com/img/logos/158x48/reform-logo-gray-900.svg");
						props.set_attribute("alt", "Reform");
						props.set_attribute("width", "158");
						props.set_attribute("height", "48");
					});
					props.child("5", Dynamic).run("img", |mut props| {
						props.styles(vec![Style::Noop("col-span-2"), Style::Noop("max-h-12"), Style::Width(Size::Full), Style::Noop("object-contain"), Style::OnScreen(Screen::Large, &[Style::Noop("col-span-1")])]);
						props.set_attribute("src", "https://tailwindui.com/img/logos/158x48/tuple-logo-gray-900.svg");
						props.set_attribute("alt", "Tuple");
						props.set_attribute("width", "158");
						props.set_attribute("height", "48");
					});
					props.child("7", Dynamic).run("img", |mut props| {
						props.styles(vec![Style::Noop("col-span-2"), Style::Noop("max-h-12"), Style::Width(Size::Full), Style::Noop("object-contain"), Style::OnScreen(Screen::Small, &[Style::Noop("col-start-2")]), Style::OnScreen(Screen::Large, &[Style::Noop("col-span-1")])]);
						props.set_attribute("src", "https://tailwindui.com/img/logos/158x48/savvycal-logo-gray-900.svg");
						props.set_attribute("alt", "SavvyCal");
						props.set_attribute("width", "158");
						props.set_attribute("height", "48");
					});
					props.child("9", Dynamic).run("img", |mut props| {
						props.styles(vec![Style::Noop("col-span-2"), Style::Noop("col-start-2"), Style::Noop("max-h-12"), Style::Width(Size::Full), Style::Noop("object-contain"), Style::OnScreen(Screen::Small, &[Style::Noop("col-start-auto")]), Style::OnScreen(Screen::Large, &[Style::Noop("col-span-1")])]);
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
