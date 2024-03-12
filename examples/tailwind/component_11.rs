use rust_web_ui::*;

pub struct Example11;

pub struct Example11Props {}

impl Default for Example11Props {
	fn default() -> Example11Props {
		Example11Props { }
	}
}

impl Widget<'_> for Example11 {
	type Props = Example11Props;

	fn render(mut ctx: Ctx<'_>, props: Example11Props) {
		ctx.styles(&[Style::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |mut props| {
			props.styles(&[Style::Noop("bg-white"), Style::Padding(Size::Exact(32))]);
			props.set_attribute("style", "min-height: 384px");
			
			props.child("1", Dynamic).run("div", |mut props| {
				props.styles(&[Style::Noop("mx-auto"), Style::Noop("max-w-7xl")]);
				
				props.child("1", Dynamic).run("div", |mut props| {
					props.styles(&[Style::OnScreen(Screen::Large, &[Style::Flex]), Style::OnScreen(Screen::Large, &[Style::ItemsCenter]), Style::OnScreen(Screen::Large, &[Style::JustifyBetween])]);
					
					props.child("1", Dynamic).run("div", |mut props| {
						props.styles(&[Style::Noop("min-w-0"), Style::Noop("flex-1")]);
						
						props.child("1", Dynamic).run("h2", |mut props| {
							props.styles(&[Style::Noop("text-2xl"), Style::FontBold, Style::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Style::OnScreen(Screen::Small, &[Style::Noop("truncate")]), Style::OnScreen(Screen::Small, &[Style::Noop("text-3xl")]), Style::OnScreen(Screen::Small, &[Style::Noop("tracking-tight")])]);
							
							props.child("0", Label).run(|props| props.text("Back End Developer"));
						});
						props.child("3", Dynamic).run("div", |mut props| {
							props.styles(&[Style::MarginTop(Size::Exact(4)), Style::Flex, Style::Noop("flex-col"), Style::OnScreen(Screen::Small, &[Style::MarginTop(Size::Exact(0))]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-row")]), Style::OnScreen(Screen::Small, &[Style::Noop("flex-wrap")]), Style::OnScreen(Screen::Small, &[Style::SpaceX(Size::Exact(24))])]);
							
							props.child("1", Dynamic).run("div", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Flex, Style::ItemsCenter, Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("1", Icon).run(|mut props| {
									props.style(&[Style::MarginRight(Size::Exact(6)), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
								});
								props.child("2", Label).run(|props| props.text("Full-time"));
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Flex, Style::ItemsCenter, Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("1", Icon).run(|mut props| {
									props.style(&[Style::MarginRight(Size::Exact(6)), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
								});
								props.child("2", Label).run(|props| props.text("Remote"));
							});
							props.child("5", Dynamic).run("div", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Flex, Style::ItemsCenter, Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("1", Icon).run(|mut props| {
									props.style(&[Style::MarginRight(Size::Exact(6)), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
								});
								props.child("2", Label).run(|props| props.text("$120k – $140k"));
							});
							props.child("7", Dynamic).run("div", |mut props| {
								props.styles(&[Style::MarginTop(Size::Exact(8)), Style::Flex, Style::ItemsCenter, Style::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("1", Icon).run(|mut props| {
									props.style(&[Style::MarginRight(Size::Exact(6)), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
								});
								props.child("2", Label).run(|props| props.text("Closing on January 9, 2020"));
							});
						});
					});
					props.child("3", Dynamic).run("div", |mut props| {
						props.styles(&[Style::MarginTop(Size::Exact(20)), Style::Flex, Style::OnScreen(Screen::Large, &[Style::MarginLeft(Size::Exact(16))]), Style::OnScreen(Screen::Large, &[Style::MarginTop(Size::Exact(0))])]);
						
						props.child("1", Dynamic).run("span", |mut props| {
							props.styles(&[Style::Noop("hidden"), Style::OnScreen(Screen::Small, &[Style::Block])]);
							
							props.child("1", Dynamic).run("button", |mut props| {
								props.set_attribute("type", "button");
								props.styles(&[Style::InlineFlex, Style::ItemsCenter, Style::Noop("rounded-md"), Style::Noop("bg-white"), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
								
								props.child("1", Icon).run(|mut props| {
									props.style(&[Style::Noop("-ml-0.5"), Style::MarginRight(Size::Exact(6)), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::TextColor(Color::Fg(44))]);
								});
								props.child("2", Label).run(|props| props.text("Edit"));
							});
						});
						props.child("3", Dynamic).run("span", |mut props| {
							props.styles(&[Style::MarginLeft(Size::Exact(12)), Style::Noop("hidden"), Style::OnScreen(Screen::Small, &[Style::Block])]);
							
							props.child("1", Dynamic).run("button", |mut props| {
								props.set_attribute("type", "button");
								props.styles(&[Style::InlineFlex, Style::ItemsCenter, Style::Noop("rounded-md"), Style::Noop("bg-white"), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::OnHover(&[Style::Color(Color::Fg(6))])]);
								
								props.child("1", Icon).run(|mut props| {
									props.style(&[Style::Noop("-ml-0.5"), Style::MarginRight(Size::Exact(6)), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::TextColor(Color::Fg(44))]);
								});
								props.child("2", Label).run(|props| props.text("View"));
							});
						});
						props.child("5", Dynamic).run("span", |mut props| {
							props.styles(&[Style::OnScreen(Screen::Small, &[Style::MarginLeft(Size::Exact(12))])]);
							
							props.child("1", Dynamic).run("button", |mut props| {
								props.set_attribute("type", "button");
								props.styles(&[Style::InlineFlex, Style::ItemsCenter, Style::Noop("rounded-md"), Style::Noop("bg-indigo-600"), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::FontSemibold, Style::Noop("text-white"), Style::Noop("shadow-sm"), Style::OnHover(&[Style::Noop("bg-indigo-500")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-2")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-offset-2")]), Style::NoopGroup("focus-visible", &[Style::Noop("outline-indigo-600")])]);
								
								props.child("1", Icon).run(|mut props| {
									props.style(&[Style::Noop("-ml-0.5"), Style::MarginRight(Size::Exact(6)), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20))]);
								});
								props.child("2", Label).run(|props| props.text("Publish"));
							});
						});
						//  Dropdown 
						props.child("9", Dynamic).run("div", |mut props| {
							props.set_attribute("x-data", "Components.menu({ open: false })");
							props.set_attribute("x-init", "init()");
							props.set_attribute("@keydown.escape.stop", "open = false; focusButton()");
							props.set_attribute("@click.away", "onClickAway($event)");
							props.styles(&[Style::Noop("relative"), Style::MarginLeft(Size::Exact(12)), Style::OnScreen(Screen::Small, &[Style::Noop("hidden")])]);
							
							props.child("1", Dynamic).run("button", |mut props| {
								props.set_attribute("type", "button");
								props.styles(&[Style::InlineFlex, Style::ItemsCenter, Style::Noop("rounded-md"), Style::Noop("bg-white"), Style::PaddingX(Size::Exact(12)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100)), Style::Noop("shadow-sm"), Style::Noop("ring-1"), Style::Noop("ring-inset"), Style::Noop("ring-gray-300"), Style::OnHover(&[Style::Noop("ring-gray-400")])]);
								props.set_attribute("id", "mobile-menu-button");
								props.set_attribute("x-ref", "button");
								props.set_attribute("@click", "onButtonClick()");
								props.set_attribute("@keyup.space.prevent", "onButtonEnter()");
								props.set_attribute("@keydown.enter.prevent", "onButtonEnter()");
								props.set_attribute("aria-expanded", "false");
								props.set_attribute("aria-haspopup", "true");
								props.set_attribute("x-bind:aria-expanded", "open.toString()");
								props.set_attribute("@keydown.arrow-up.prevent", "onArrowUp()");
								props.set_attribute("@keydown.arrow-down.prevent", "onArrowDown()");
								
								props.child("0", Label).run(|props| props.text("More"));
								props.child("1", Icon).run(|mut props| {
									props.style(&[Style::Noop("-mr-1"), Style::MarginLeft(Size::Exact(6)), Style::Width(Size::Exact(20)), Style::Width(Size::Exact(20)), Style::TextColor(Color::Fg(44))]);
								});
							});
							props.child("3", Dynamic).run("div", |mut props| {
								props.set_attribute("x-show", "open");
								props.set_attribute("x-transition:enter", "transition ease-out duration-200");
								props.set_attribute("x-transition:enter-start", "transform opacity-0 scale-95");
								props.set_attribute("x-transition:enter-end", "transform opacity-100 scale-100");
								props.set_attribute("x-transition:leave", "transition ease-in duration-75");
								props.set_attribute("x-transition:leave-start", "transform opacity-100 scale-100");
								props.set_attribute("x-transition:leave-end", "transform opacity-0 scale-95");
								props.styles(&[Style::Noop("absolute"), Style::Noop("right-0"), Style::Noop("z-10"), Style::Noop("-mr-1"), Style::MarginTop(Size::Exact(8)), Style::Width(Size::Exact(192)), Style::Noop("origin-top-right"), Style::Noop("rounded-md"), Style::Noop("bg-white"), Style::PaddingY(Size::Exact(4)), Style::Noop("shadow-lg"), Style::Noop("ring-1"), Style::Noop("ring-black"), Style::Noop("ring-opacity-5"), Style::OnFocus(&[Style::Noop("outline-none")])]);
								props.set_attribute("x-ref", "menu-items");
								props.set_attribute("x-description", "Dropdown menu, show/hide based on menu state.");
								props.set_attribute("x-bind:aria-activedescendant", "activeDescendant");
								props.set_attribute("role", "menu");
								props.set_attribute("aria-orientation", "vertical");
								props.set_attribute("aria-labelledby", "mobile-menu-button");
								props.set_attribute("tabindex", "-1");
								props.set_attribute("@keydown.arrow-up.prevent", "onArrowUp()");
								props.set_attribute("@keydown.arrow-down.prevent", "onArrowDown()");
								props.set_attribute("@keydown.tab", "open = false");
								props.set_attribute("@keydown.enter.prevent", "open = false; focusButton()");
								props.set_attribute("@keyup.space.prevent", "open = false; focusButton()");
								
								props.child("1", Dynamic).run("a", |mut props| {
									props.set_attribute("href", "#");
									props.styles(&[Style::Block, Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
									props.set_attribute("x-state:on", "Active");
									props.set_attribute("x-state:off", "Not Active");
									props.set_attribute(":class", "{ 'bg-gray-100': activeIndex === 0 }");
									props.set_attribute("role", "menuitem");
									props.set_attribute("tabindex", "-1");
									props.set_attribute("id", "mobile-menu-item-0");
									props.set_attribute("@mouseenter", "onMouseEnter($event)");
									props.set_attribute("@mousemove", "onMouseMove($event, 0)");
									props.set_attribute("@mouseleave", "onMouseLeave($event)");
									props.set_attribute("@click", "open = false; focusButton()");
									
									props.child("0", Label).run(|props| props.text("Edit"));
								});
								props.child("3", Dynamic).run("a", |mut props| {
									props.set_attribute("href", "#");
									props.styles(&[Style::Block, Style::PaddingX(Size::Exact(16)), Style::PaddingY(Size::Exact(8)), Style::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
									props.set_attribute(":class", "{ 'bg-gray-100': activeIndex === 1 }");
									props.set_attribute("role", "menuitem");
									props.set_attribute("tabindex", "-1");
									props.set_attribute("id", "mobile-menu-item-1");
									props.set_attribute("@mouseenter", "onMouseEnter($event)");
									props.set_attribute("@mousemove", "onMouseMove($event, 1)");
									props.set_attribute("@mouseleave", "onMouseLeave($event)");
									props.set_attribute("@click", "open = false; focusButton()");
									
									props.child("0", Label).run(|props| props.text("View"));
								});
							});
						});
					});
				});
			});
		});
	}
}
