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
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[NoStyle::Noop("bg-white"), Style::Padding(32)]);
			props.set_attribute("style", "min-height: 384px");
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("max-w-7xl")]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.styles(&[Screen::Large(&[Style::Flex]), Screen::Large(&[Style::ItemsCenter]), Screen::Large(&[Style::JustifyBetween])]);
					
					props.child("1", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("min-w-0"), NoStyle::Noop("flex-1")]);
						
						props.child("1", Dynamic).run("h2", |props| {
							props.styles(&[NoStyle::Noop("text-2xl"), Style::FontBold, NoStyle::Noop("leading-7"), Style::TextColor(Color::Fg(100)), Screen::Small(&[NoStyle::Noop("truncate")]), Screen::Small(&[NoStyle::Noop("text-3xl")]), Screen::Small(&[NoStyle::Noop("tracking-tight")])]);
							
							props.child("0", Label).run(|props| props.set_text("Back End Developer"));
						});
						props.child("3", Dynamic).run("div", |props| {
							props.styles(&[Style::MarginTop(4), Style::Flex, NoStyle::Noop("flex-col"), Screen::Small(&[Style::MarginTop(0)]), Screen::Small(&[NoStyle::Noop("flex-row")]), Screen::Small(&[NoStyle::Noop("flex-wrap")]), Screen::Small(&[Style::SpaceX(24)])]);
							
							props.child("1", Dynamic).run("div", |props| {
								props.styles(&[Style::MarginTop(8), Style::Flex, Style::ItemsCenter, NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("1", Icon).run(|props| {
									props.style(&[Style::MarginRight(6), Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
								});
								props.child("2", Label).run(|props| props.set_text("Full-time"));
							});
							props.child("3", Dynamic).run("div", |props| {
								props.styles(&[Style::MarginTop(8), Style::Flex, Style::ItemsCenter, NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("1", Icon).run(|props| {
									props.style(&[Style::MarginRight(6), Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
								});
								props.child("2", Label).run(|props| props.set_text("Remote"));
							});
							props.child("5", Dynamic).run("div", |props| {
								props.styles(&[Style::MarginTop(8), Style::Flex, Style::ItemsCenter, NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("1", Icon).run(|props| {
									props.style(&[Style::MarginRight(6), Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
								});
								props.child("2", Label).run(|props| props.set_text("$120k – $140k"));
							});
							props.child("7", Dynamic).run("div", |props| {
								props.styles(&[Style::MarginTop(8), Style::Flex, Style::ItemsCenter, NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(56))]);
								
								props.child("1", Icon).run(|props| {
									props.style(&[Style::MarginRight(6), Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0"), Style::TextColor(Color::Fg(44))]);
								});
								props.child("2", Label).run(|props| props.set_text("Closing on January 9, 2020"));
							});
						});
					});
					props.child("3", Dynamic).run("div", |props| {
						props.styles(&[Style::MarginTop(20), Style::Flex, Screen::Large(&[Style::MarginLeft(16)]), Screen::Large(&[Style::MarginTop(0)])]);
						
						props.child("1", Dynamic).run("span", |props| {
							props.styles(&[NoStyle::Noop("hidden"), Screen::Small(&[Style::Block])]);
							
							props.child("1", Dynamic).run("button", |props| {
								props.set_attribute("type", "button");
								props.styles(&[Style::InlineFlex, Style::ItemsCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-white"), Style::PaddingX(12), Style::PaddingY(8), NoStyle::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), Action::Hover(&[Style::Color(Color::Fg(6))])]);
								
								props.child("1", Icon).run(|props| {
									props.style(&[NoStyle::Noop("-ml-0.5"), Style::MarginRight(6), Style::Width(20), Style::Width(20), Style::TextColor(Color::Fg(44))]);
								});
								props.child("2", Label).run(|props| props.set_text("Edit"));
							});
						});
						props.child("3", Dynamic).run("span", |props| {
							props.styles(&[Style::MarginLeft(12), NoStyle::Noop("hidden"), Screen::Small(&[Style::Block])]);
							
							props.child("1", Dynamic).run("button", |props| {
								props.set_attribute("type", "button");
								props.styles(&[Style::InlineFlex, Style::ItemsCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-white"), Style::PaddingX(12), Style::PaddingY(8), NoStyle::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), Action::Hover(&[Style::Color(Color::Fg(6))])]);
								
								props.child("1", Icon).run(|props| {
									props.style(&[NoStyle::Noop("-ml-0.5"), Style::MarginRight(6), Style::Width(20), Style::Width(20), Style::TextColor(Color::Fg(44))]);
								});
								props.child("2", Label).run(|props| props.set_text("View"));
							});
						});
						props.child("5", Dynamic).run("span", |props| {
							props.styles(&[Screen::Small(&[Style::MarginLeft(12)])]);
							
							props.child("1", Dynamic).run("button", |props| {
								props.set_attribute("type", "button");
								props.styles(&[Style::InlineFlex, Style::ItemsCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-indigo-600"), Style::PaddingX(12), Style::PaddingY(8), NoStyle::Noop("text-sm"), Style::FontSemibold, NoStyle::Noop("text-white"), NoStyle::Noop("shadow-sm"), Action::Hover(&[NoStyle::Noop("bg-indigo-500")]), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-2")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-offset-2")), NoStyle::NoopGroup("focus-visible", NoStyle::Noop("outline-indigo-600"))]);
								
								props.child("1", Icon).run(|props| {
									props.style(&[NoStyle::Noop("-ml-0.5"), Style::MarginRight(6), Style::Width(20), Style::Width(20)]);
								});
								props.child("2", Label).run(|props| props.set_text("Publish"));
							});
						});
						//  Dropdown 
						props.child("9", Dynamic).run("div", |props| {
							props.set_attribute("x-data", "Components.menu({ open: false })");
							props.set_attribute("x-init", "init()");
							props.set_attribute("@keydown.escape.stop", "open = false; focusButton()");
							props.set_attribute("@click.away", "onClickAway($event)");
							props.styles(&[NoStyle::Noop("relative"), Style::MarginLeft(12), Screen::Small(&[NoStyle::Noop("hidden")])]);
							
							props.child("1", Dynamic).run("button", |props| {
								props.set_attribute("type", "button");
								props.styles(&[Style::InlineFlex, Style::ItemsCenter, NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-white"), Style::PaddingX(12), Style::PaddingY(8), NoStyle::Noop("text-sm"), Style::FontSemibold, Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), Action::Hover(&[NoStyle::Noop("ring-gray-400")])]);
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
								
								props.child("0", Label).run(|props| props.set_text("More"));
								props.child("1", Icon).run(|props| {
									props.style(&[NoStyle::Noop("-mr-1"), Style::MarginLeft(6), Style::Width(20), Style::Width(20), Style::TextColor(Color::Fg(44))]);
								});
							});
							props.child("3", Dynamic).run("div", |props| {
								props.set_attribute("x-show", "open");
								props.set_attribute("x-transition:enter", "transition ease-out duration-200");
								props.set_attribute("x-transition:enter-start", "transform opacity-0 scale-95");
								props.set_attribute("x-transition:enter-end", "transform opacity-100 scale-100");
								props.set_attribute("x-transition:leave", "transition ease-in duration-75");
								props.set_attribute("x-transition:leave-start", "transform opacity-100 scale-100");
								props.set_attribute("x-transition:leave-end", "transform opacity-0 scale-95");
								props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("right-0"), NoStyle::Noop("z-10"), NoStyle::Noop("-mr-1"), Style::MarginTop(8), Style::Width(192), NoStyle::Noop("origin-top-right"), NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-white"), Style::PaddingY(4), NoStyle::Noop("shadow-lg"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-black"), NoStyle::Noop("ring-opacity-5"), Action::Hover(&[NoStyle::Noop("outline-none")])]);
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
								
								props.child("1", Dynamic).run("a", |props| {
									props.set_attribute("href", "#");
									props.styles(&[Style::Block, Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
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
									
									props.child("0", Label).run(|props| props.set_text("Edit"));
								});
								props.child("3", Dynamic).run("a", |props| {
									props.set_attribute("href", "#");
									props.styles(&[Style::Block, Style::PaddingX(16), Style::PaddingY(8), NoStyle::Noop("text-sm"), Style::TextColor(Color::Fg(78))]);
									props.set_attribute(":class", "{ 'bg-gray-100': activeIndex === 1 }");
									props.set_attribute("role", "menuitem");
									props.set_attribute("tabindex", "-1");
									props.set_attribute("id", "mobile-menu-item-1");
									props.set_attribute("@mouseenter", "onMouseEnter($event)");
									props.set_attribute("@mousemove", "onMouseMove($event, 1)");
									props.set_attribute("@mouseleave", "onMouseLeave($event)");
									props.set_attribute("@click", "open = false; focusButton()");
									
									props.child("0", Label).run(|props| props.set_text("View"));
								});
							});
						});
					});
				});
			});
		});
	}
}
