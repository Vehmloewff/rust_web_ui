use rust_web_ui::*;

pub struct Example15;

pub struct Example15Props {}

impl Default for Example15Props {
	fn default() -> Example15Props {
		Example15Props { }
	}
}

impl Widget<'_> for Example15 {
	type Props = Example15Props;

	fn render(mut ctx: Ctx<'_>, props: Example15Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Flex, Style::ItemsCenter, Style::JustifyCenter, Style::Noop("bg-white"), Style::Padding(Size::Exact(32))]);
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("mx-auto"), Style::Width(Size::Full), Style::Noop("max-w-xs")]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.child("1", Dynamic).run("label", |mut props| {
						props.set_attribute("for", "price");
						props.styles(&[Style::Block, Style::Noop("text-sm"), Style::Noop("font-medium"), Style::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
						
						props.child("0", Label).run(|props| props.text("Price"));
					});
					props.child("3", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("relative"), Style::MarginTop(Size::Exact(8)), Style::Noop("rounded-md"), Style::Noop("shadow-sm")]);
						
						props.child("1", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("pointer-events-none"), Style::Noop("absolute"), Style::Noop("inset-y-0"), Style::Noop("left-0"), Style::Flex, Style::ItemsCenter, Style::PaddingLeft(Size::Exact(12))]);
							
							props.child("1", Dynamic).run("span", |mut props| {
								props.styles(&[Style::TextColor(Color::Fg(56)), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")])]);
								
								props.child("0", Label).run(|props| props.text("$"));
							});
						});
						props.child("3", Dynamic).run("input", |mut props| {
							props.set_attribute("type", "text");
							props.set_attribute("name", "price");
							props.set_attribute("id", "price");
							props.styles(&[Style::Block, Style::Width(Size::Full), Style::Noop("rounded-md"), Style::Noop("border-0"), Style::PaddingY(Size::Exact(6)), Style::PaddingLeft(Size::Exact(28)), Style::PaddingRight(Size::Exact(80)), Style::TextColor(Color::Fg(100)), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::NoopGroup("placeholder", &[Style::TextColor(Color::Fg(44))]), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-inset")]), Style::OnFocus(&[Style::Noop("ring-indigo-600")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")]), Style::OnScreen(Screen::Small, &[Style::Noop("leading-6")])]);
							props.set_attribute("placeholder", "0.00");
						});
						props.child("5", Dynamic).run("div", |mut props| {
							props.styles(&[Style::Noop("absolute"), Style::Noop("inset-y-0"), Style::Noop("right-0"), Style::Flex, Style::ItemsCenter]);
							
							props.child("1", Dynamic).run("label", |mut props| {
								props.set_attribute("for", "currency");
								props.styles(&[Style::Noop("sr-only")]);
								
								props.child("0", Label).run(|props| props.text("Currency"));
							});
							props.child("3", Dynamic).run("select", |mut props| {
								props.set_attribute("id", "currency");
								props.set_attribute("name", "currency");
								props.styles(&[Style::Width(Size::Full), Style::Noop("rounded-md"), Style::Noop("border-0"), Style::Noop("bg-transparent"), Style::PaddingY(Size::Exact(0)), Style::PaddingLeft(Size::Exact(8)), Style::PaddingRight(Size::Exact(28)), Style::TextColor(Color::Fg(56)), Style::OnFocus(&[Style::Noop("ring-2")]), Style::OnFocus(&[Style::Noop("ring-inset")]), Style::OnFocus(&[Style::Noop("ring-indigo-600")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-sm")])]);
								
								props.child("1", Dynamic).run("option", |mut props| {
									props.child("0", Label).run(|props| props.text("USD"));
								});
								props.child("3", Dynamic).run("option", |mut props| {
									props.child("0", Label).run(|props| props.text("CAD"));
								});
								props.child("5", Dynamic).run("option", |mut props| {
									props.child("0", Label).run(|props| props.text("EUR"));
								});
							});
						});
					});
				});
			});
		});
	}
}
